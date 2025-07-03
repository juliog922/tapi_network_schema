use gateway::{
    CONFIG_HASH, RateLimiter, config::Config, parse_log_level, routes::proxy::proxy_handler,
};

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use actix_cors::Cors;
use actix_web::http::Method;
use actix_web::middleware::Logger;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, guard, web};
use env_logger::Env;
use sha2::{Digest, Sha256};
use tokio::time::interval;

/// Embedded contents of the configuration file at compile time.
const CONFIG_YAML: &str = include_str!("../config/config.yaml");

/// Entry point for the API Gateway service.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Validate the hash of the config file against the value from build time
    let mut hasher = Sha256::new();
    hasher.update(CONFIG_YAML);
    let runtime_hash = format!("{:x}", hasher.finalize());

    if runtime_hash != CONFIG_HASH {
        eprintln!("The config.yaml file has changed since the last build.");
        std::process::exit(1);
    }

    // Parse the embedded YAML configuration
    let config: Config = serde_yaml::from_str(CONFIG_YAML).expect("Invalid config.yaml format");

    // Ensure no duplicate route paths/methods
    let mut paths_seen = HashSet::new();
    for route in &config.routes {
        if !paths_seen.insert((&route.path, &route.method)) {
            panic!("Duplicate route detected: {} {}", route.method, route.path);
        }
    }

    // Initialize logging using level from configuration
    let level = parse_log_level(&config.global.logging.level);
    env_logger::Builder::from_env(Env::default())
        .filter_level(level)
        .init();

    // Shared rate-limiting state across routes
    let rate_limiter: RateLimiter = Arc::new(Mutex::new(HashMap::new()));
    let rl_clone = rate_limiter.clone();

    // Background task to clear the rate-limiter every minute
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(60));
        loop {
            ticker.tick().await;
            rl_clone.lock().unwrap().clear();
        }
    });

    log::info!("Gateway running at http://0.0.0.0:8081");

    // Start HTTP server
    HttpServer::new(move || {
        // Configure CORS
        let mut cors = Cors::default();
        if config.global.cors.enabled {
            if config
                .global
                .cors
                .allowed_origins
                .contains(&"*".to_string())
            {
                cors = cors.send_wildcard();
            } else {
                for origin in &config.global.cors.allowed_origins {
                    cors = cors.allowed_origin(origin);
                }
            }

            let methods: Vec<Method> = config
                .global
                .cors
                .allowed_methods
                .iter()
                .filter_map(|m| m.parse().ok())
                .collect();

            cors = cors.allowed_methods(methods);
            cors = cors.allow_any_header();
        }

        // Create the Actix app
        let mut app = App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(rate_limiter.clone()));

        // Register proxy routes from config
        for route in &config.routes {
            let method = Method::from_bytes(route.method.as_bytes()).unwrap();
            let route_data = web::Data::new(route.clone());
            let config_data = web::Data::new(config.clone());
            let rate_limiter_data = web::Data::new(rate_limiter.clone());

            app = app.route(
                &route.path,
                web::route()
                    .guard(guard::fn_guard({
                        let method = method.clone();
                        move |ctx| ctx.head().method == method
                    }))
                    .to({
                        let route_data = route_data.clone();
                        move |req: HttpRequest, body: web::Bytes| {
                            proxy_handler(
                                req,
                                body,
                                route_data.clone(),
                                rate_limiter_data.clone(),
                                config_data.clone(),
                            )
                        }
                    }),
            );
        }

        // Fallback handler for unmatched routes
        app.default_service(web::to(|| async {
            HttpResponse::NotFound().body("Not found")
        }))
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, HttpResponse, HttpServer, test, web};
    use gateway::config::{CorsSettings, GlobalSettings, LoggingSettings, Route};
    use std::net::TcpListener;
    use tokio::task;

    /// Tests that the API Gateway correctly proxies a request to an upstream service.
    ///
    /// This test sets up a mock backend service that listens on `/`, and verifies
    /// that when a request is sent to the gateway at `/api`, it is forwarded to
    /// the upstream service as configured in the `Route` definition.
    ///
    /// Expected behavior:
    /// - The gateway matches the path `/api`
    /// - Forwards the request body to the upstream
    /// - Returns the upstream response unchanged
    #[actix_web::test]
    async fn test_gateway_proxies_to_mock_backend() {
        // Start a mock backend service on a random local port
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let upstream_url = format!("http://{}", addr);
        println!("🧪 Mock backend escuchando en: {}", upstream_url);

        // Spawn the upstream HTTP server responding at "/"
        task::spawn(async move {
            HttpServer::new(|| {
                App::new().route(
                    "/",
                    web::post().to(|| async {
                        println!("✅ Mock backend recibió la petición");
                        HttpResponse::Ok().body("Hello from test backend")
                    }),
                )
            })
            .listen(listener)
            .unwrap()
            .run()
            .await
            .unwrap();
        });

        // Define a gateway route that maps "/api" to the upstream "/"
        let route = Route {
            name: "test_route".to_string(),
            path: "/api".to_string(),
            method: "POST".to_string(),
            upstream_url: upstream_url.clone(),
            auth_required: false,
            rate_limit: None,
        };

        let config = Config {
            routes: vec![route.clone()],
            global: GlobalSettings {
                cors: CorsSettings {
                    enabled: false,
                    allowed_origins: vec![],
                    allowed_methods: vec![],
                },
                timeout: 30,
                logging: LoggingSettings {
                    level: "debug".to_string(),
                },
            },
        };

        let rate_limiter: RateLimiter = Arc::new(Mutex::new(HashMap::new()));

        // Set up the test version of the gateway with the route configured
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(config.clone()))
                .app_data(web::Data::new(rate_limiter.clone()))
                .route(
                    &route.path,
                    web::route()
                        .guard(guard::fn_guard({
                            let method = Method::from_bytes(route.method.as_bytes()).unwrap();
                            move |ctx| ctx.head().method == method
                        }))
                        .to({
                            let route_data = web::Data::new(route.clone());
                            let config_data = web::Data::new(config.clone());
                            let rate_data = web::Data::new(rate_limiter.clone());
                            move |req, body| {
                                proxy_handler(
                                    req,
                                    body,
                                    route_data.clone(),
                                    rate_data.clone(),
                                    config_data.clone(),
                                )
                            }
                        }),
                ),
        )
        .await;

        // Simulate a POST request to the gateway at /api
        let req = test::TestRequest::post()
            .uri("/api")
            .set_payload("test payload")
            .to_request();

        // Execute the request and capture the response
        let resp = test::call_service(&app, req).await;
        let status = resp.status();
        let body = test::read_body(resp).await;

        // Assert the response matches the upstream response
        assert!(status.is_success(), "Expected 2xx success response");
        assert_eq!(body, web::Bytes::from_static(b"Hello from test backend"));
    }
}

use crate::{
    AppError, RateLimiter,
    models::config::{Config, Route},
};

use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError, web};
use awc::Client;
use std::time::{Duration, Instant};
use tokio::time::timeout;

/// Handles proxying incoming HTTP requests to their configured upstream destination.
///
/// This handler performs:
/// - Route matching
/// - Rate limiting (if configured)
/// - Optional authentication header injection
/// - Timeout handling
/// - Request forwarding using `awc::Client`
///
/// Returns a `Responder` with the upstream's response or an appropriate gateway error.
pub async fn proxy_handler(
    req: HttpRequest,
    body: web::Bytes,
    route: web::Data<Route>,
    limiter: web::Data<RateLimiter>,
    config: web::Data<Config>,
) -> impl Responder {
    // Apply configured timeout per request
    let duration = Duration::from_secs(config.global.timeout);

    let result = timeout(duration, async {
        let client = Client::default();
        let mut target_url = route.upstream_url.clone();

        for (key, value) in req.match_info().iter() {
            let ph = format!("{{{}}}", key);
            target_url = target_url.replace(&ph, value);
        }

        log::info!("Forwarding request to upstream: {}", target_url);

        let mut fwd_req = client.request(req.method().clone(), target_url);

        // Copy original request headers
        for (name, value) in req.headers().iter() {
            fwd_req = fwd_req.insert_header((name.clone(), value.clone()));
        }

        // Inject static auth headers if required by route
        if route.auth_required {
            fwd_req = fwd_req
                .insert_header(("user", "test"))
                .insert_header(("role", "admin"));
        }

        // Rate limiting logic (per IP + route)
        if let Some(limit) = &route.rate_limit {
            let ip = req
                .peer_addr()
                .map(|addr| addr.ip().to_string())
                .unwrap_or_default();
            let key = format!("{}:{}", route.name, ip);
            let mut guard = limiter.lock().unwrap();
            let (count, _) = guard
                .entry(key.clone())
                .and_modify(|(c, _)| *c += 1)
                .or_insert((1, Instant::now()));
            if *count > limit.requests_per_minute {
                return HttpResponse::TooManyRequests().body("Too many requests");
            }
        }

        // Forward request to upstream
        match fwd_req.send_body(body.clone()).await {
            Ok(mut resp) => {
                let status = resp.status();
                let body_bytes = resp
                    .body()
                    .await
                    .unwrap_or_else(|_| web::Bytes::from_static(b""));
                HttpResponse::build(status).body(body_bytes)
            }
            Err(e) => {
                log::error!("Upstream request failed: {}", e);
                AppError::Upstream(e).error_response()
            }
        }
    })
    .await;

    // Convert `timeout` result into a response
    result.unwrap_or_else(|_| AppError::Unexpected("Request timed out".into()).error_response())
}

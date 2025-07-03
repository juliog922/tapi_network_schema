use serde::Deserialize;

/// Top-level application configuration structure, deserialized from `config.yaml`.
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// List of route definitions the gateway should handle.
    pub routes: Vec<Route>,

    /// Global settings applicable to all routes.
    pub global: GlobalSettings,
}

/// Represents a single route in the API gateway.
///
/// Each route maps an incoming request to an upstream service, and
/// may include optional authentication and rate limiting policies.
#[derive(Debug, Deserialize, Clone)]
pub struct Route {
    /// Unique name used internally to identify the route.
    pub name: String,

    /// The HTTP path to match (e.g. `/api`, `/health`).
    pub path: String,

    /// The HTTP method to match (e.g. `GET`, `POST`).
    pub method: String,

    /// The destination URL to forward the request to.
    pub upstream_url: String,

    /// Whether this route requires authentication.
    pub auth_required: bool,

    /// Optional rate limiting policy for the route.
    pub rate_limit: Option<RateLimit>,
}

/// Rate limiting policy applied to a route.
#[derive(Debug, Deserialize, Clone)]
pub struct RateLimit {
    /// Maximum number of requests allowed per minute from a single client.
    pub requests_per_minute: u32,
}

/// Configuration for global CORS, timeout, and logging behavior.
#[derive(Debug, Deserialize, Clone)]
pub struct GlobalSettings {
    /// CORS policy to apply to all routes.
    pub cors: CorsSettings,

    /// Timeout (in seconds) applied to each request before failing.
    pub timeout: u64,

    /// Logging verbosity and format options.
    pub logging: LoggingSettings,
}

/// Cross-Origin Resource Sharing (CORS) policy settings.
#[derive(Debug, Deserialize, Clone)]
pub struct CorsSettings {
    /// Whether CORS is enabled for the gateway.
    pub enabled: bool,

    /// Allowed origin domains (e.g., `["*"]` or specific domains).
    pub allowed_origins: Vec<String>,

    /// Allowed HTTP methods for CORS preflight and actual requests.
    pub allowed_methods: Vec<String>,
}

/// Logging configuration for the gateway.
#[derive(Debug, Deserialize, Clone)]
pub struct LoggingSettings {
    /// Logging level: e.g., `info`, `debug`, `warn`, `error`.
    pub level: String,
}

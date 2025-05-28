//! Shared crate-level declarations for the gateway.

mod config_hash;
mod error;
mod models;
pub mod routes;
mod utils;

// Public exports used across the application.
pub use config_hash::CONFIG_HASH;
pub use error::AppError;
pub use models::config;
pub use utils::parse_log_level;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// In-memory rate limiter shared across requests.
pub type RateLimiter = Arc<Mutex<HashMap<String, (u32, Instant)>>>;

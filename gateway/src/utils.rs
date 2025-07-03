use log::{LevelFilter, warn};

/// Parses a string log level into a `LevelFilter`.
///
/// Accepts values such as "debug", "info", "warn", "error", "trace", or "off".
/// Falls back to `LevelFilter::Info` if the input is unrecognized.
pub fn parse_log_level(level: &str) -> LevelFilter {
    match level.to_lowercase().as_str() {
        "debug" => LevelFilter::Debug,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "trace" => LevelFilter::Trace,
        "off" => LevelFilter::Off,
        "info" => LevelFilter::Info,
        unknown => {
            // Warn about fallback to default level
            warn!("Unrecognized log level '{}', defaulting to 'info'", unknown);
            LevelFilter::Info
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_log_level() {
        assert_eq!(parse_log_level("debug"), LevelFilter::Debug);
        assert_eq!(parse_log_level("INFO"), LevelFilter::Info);
        assert_eq!(parse_log_level("Warn"), LevelFilter::Warn);
        assert_eq!(parse_log_level("ERROR"), LevelFilter::Error);
        assert_eq!(parse_log_level("trace"), LevelFilter::Trace);
        assert_eq!(parse_log_level("off"), LevelFilter::Off);
        assert_eq!(parse_log_level("verbose"), LevelFilter::Info);
        assert_eq!(parse_log_level("invalid"), LevelFilter::Info);
    }
}

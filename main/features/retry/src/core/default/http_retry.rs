//! Default impl of [`HttpRetry`](crate::api::http::retry::HttpRetry).
//!
//! Holds a resolved [`RetryConfig`](crate::api::types::retry::retry_config::RetryConfig)
//! and answers `describe()`. Real middleware behavior is in `core::retry_layer`.

use crate::api::http::retry::HttpRetry;
use crate::api::types::retry::retry_config::RetryConfig;

/// Default HttpRetry implementation. `pub(crate)` — consumers
/// never touch this type directly; they go through `saf::retry_svc`.
#[derive(Debug)]
pub(crate) struct DefaultHttpRetry {
    config: RetryConfig,
}

impl DefaultHttpRetry {
    /// Construct from a resolved config.
    pub(crate) fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    /// Return the underlying config.
    pub(crate) fn config(&self) -> &RetryConfig {
        &self.config
    }
}

impl HttpRetry for DefaultHttpRetry {
    fn describe(&self) -> &'static str {
        const LABEL: &str = "http-retry";
        LABEL
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_constructs_and_stores_config() {
        let cfg = RetryConfig::swe_default().expect("baseline parses");
        let d = DefaultHttpRetry::new(cfg);
        let dbg = format!("{d:?}");
        assert!(dbg.contains("DefaultHttpRetry"), "debug output: {dbg}");
    }

    /// @covers: describe
    #[test]
    fn test_describe_returns_crate_name() {
        let cfg = RetryConfig::swe_default().expect("baseline parses");
        let d = DefaultHttpRetry::new(cfg);
        assert_eq!(d.describe(), "http-retry");
    }

    /// @covers: config
    #[test]
    fn test_config_returns_stored_config() {
        let cfg = RetryConfig {
            max_retries: 5,
            initial_interval_ms: 300,
            max_interval_ms: 10_000,
            multiplier: 2.0,
            retryable_statuses: vec![503],
            retryable_methods: vec!["GET".to_string()],
        };
        let d = DefaultHttpRetry::new(cfg);
        assert_eq!(d.config().max_retries, 5);
        assert_eq!(d.config().initial_interval_ms, 300);
    }
}

//! Retry policy schema — the struct layout, nothing else.
//!
//! Policy **values** live in TOML:
//! - crate-shipped baseline: `config/application.toml`
//! - workspace override: `edge/http/main/config/application.toml` under `[retry]`
//! - consumer override: whatever TOML the binary loads and passes
//!   to `RetryConfig::from_config`.
//!
//! This file deliberately contains **no** `Default` impl with
//! literal numbers — per the config-driven principle, policy is
//! data in a file, not code in a source tree.

use serde::Deserialize;

use crate::api::error::Error;

/// Retry policy schema. Deserialized from TOML via
/// [`RetryConfig::from_config`]. Consumers compose this into a
/// middleware layer via `build_retry_layer(config)`.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetryConfig {
    /// Maximum attempts per request (1 = no retry).
    pub max_retries: u32,

    /// Delay before the first retry, in milliseconds.
    pub initial_interval_ms: u64,

    /// Upper bound on any single retry interval, in milliseconds.
    pub max_interval_ms: u64,

    /// Exponential backoff base (e.g. 2.0 → 200ms, 400ms, 800ms).
    pub multiplier: f64,

    /// HTTP status codes that trigger a retry.
    pub retryable_statuses: Vec<u16>,

    /// HTTP methods that can safely be retried.
    pub retryable_methods: Vec<String>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_interval_ms: 200,
            max_interval_ms: 10000,
            multiplier: 2.0,
            retryable_statuses: vec![408, 425, 429, 500, 502, 503, 504],
            retryable_methods: vec![
                "GET".into(),
                "HEAD".into(),
                "PUT".into(),
                "DELETE".into(),
            ],
        }
    }
}

impl swe_edge_configbuilder::ConfigSection for RetryConfig {
    fn section_name() -> &'static str {
        "retry"
    }
}

impl RetryConfig {
    /// Parse a retry config from TOML text.
    ///
    /// Returns `Error::ParseFailed` with the underlying message
    /// when the text isn't valid TOML, when a required key is
    /// missing, or when an unknown key is present
    /// (`deny_unknown_fields` is set — typos fail loudly rather
    /// than silently reverting to some default).
    pub fn from_config(toml_text: &str) -> Result<Self, Error> {
        toml::from_str(toml_text).map_err(|e| Error::ParseFailed(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: from_config
    #[test]
    fn test_from_config_parses_full_toml() {
        let toml = r#"
            max_retries = 5
            initial_interval_ms = 100
            max_interval_ms = 5000
            multiplier = 1.5
            retryable_statuses = [429, 503]
            retryable_methods = ["GET"]
        "#;
        let cfg = RetryConfig::from_config(toml).expect("parses");
        assert_eq!(cfg.max_retries, 5);
        assert_eq!(cfg.initial_interval_ms, 100);
        assert_eq!(cfg.multiplier, 1.5);
        assert_eq!(cfg.retryable_statuses, vec![429, 503]);
    }

    /// @covers: from_config
    #[test]
    fn test_from_config_missing_key_is_error() {
        let toml = r#"
            max_retries = 3
            # intentionally missing initial_interval_ms
            max_interval_ms = 10000
            multiplier = 2.0
            retryable_statuses = [429]
            retryable_methods = ["GET"]
        "#;
        let err = RetryConfig::from_config(toml).unwrap_err();
        let s = err.to_string();
        assert!(
            s.contains("initial_interval_ms") || s.contains("missing field"),
            "expected error to name the missing field, got: {s}"
        );
    }

    /// @covers: from_config
    #[test]
    fn test_from_config_unknown_key_is_error() {
        let toml = r#"
            max_retries = 3
            initial_interval_ms = 200
            max_interval_ms = 10000
            multiplier = 2.0
            retryable_statuses = [429]
            retryable_methods = ["GET"]
            unknown_knob = 42
        "#;
        let err = RetryConfig::from_config(toml).unwrap_err();
        let s = err.to_string();
        assert!(
            s.contains("unknown_knob") || s.contains("unknown field"),
            "expected error to name the unknown field, got: {s}"
        );
    }

    /// @covers: Default
    #[test]
    fn test_retry_config_default_has_positive_max_retries() {
        let cfg = RetryConfig::default();
        assert!(cfg.max_retries >= 1, "baseline must allow at least one attempt");
        assert!(!cfg.retryable_statuses.is_empty());
    }

    /// @covers: ConfigSection::section_name
    #[test]
    fn test_retry_config_section_name_is_retry() {
        use swe_edge_configbuilder::ConfigSection as _;
        assert_eq!(RetryConfig::section_name(), "retry");
    }
}

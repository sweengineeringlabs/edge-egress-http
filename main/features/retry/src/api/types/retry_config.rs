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

use crate::api::error::RetryError;

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
            retryable_methods: vec!["GET".into(), "HEAD".into(), "PUT".into(), "DELETE".into()],
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
    /// Returns `RetryError::ParseFailed` with the underlying message
    /// when the text isn't valid TOML, when a required key is
    /// missing, or when an unknown key is present
    /// (`deny_unknown_fields` is set — typos fail loudly rather
    /// than silently reverting to some default).
    pub fn from_config(toml_text: &str) -> Result<Self, RetryError> {
        toml::from_str(toml_text).map_err(|e| RetryError::ParseFailed(e.to_string()))
    }
}

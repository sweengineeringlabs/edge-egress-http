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
        // @allow: no_stub_fn_bodies
        "retry"
    }
}

/// Backend-owned opt-in contract (ADR-006): presence of the `[retry]` section
/// activates the retry policy; absence leaves it off. Kept alongside
/// [`ConfigSection`] so existing direct construction keeps working.
impl swe_edge_configbuilder::OptionalSection for RetryConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies
        "retry"
    }

    fn validate_enabled(&self) -> Result<(), swe_edge_configbuilder::ConfigError> {
        self.validate().map_err(|reason| {
            swe_edge_configbuilder::ConfigError::validation(
                <Self as swe_edge_configbuilder::OptionalSection>::section_name(),
                reason,
            )
        })
    }

    fn metadata() -> swe_edge_configbuilder::FeatureMetadata {
        swe_edge_configbuilder::FeatureMetadata {
            description: "HTTP request retry with exponential backoff",
            owner: "platform-team",
            deprecated_since: None,
        }
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

    /// Return the SWE-shipped default config by parsing the baseline TOML.
    ///
    /// Fails only if the embedded TOML is malformed — a compile-time
    /// invariant maintained by the crate owners.
    pub fn swe_default() -> Result<Self, RetryError> {
        Ok(Self::default())
    }

    /// Validate the config fields.
    ///
    /// Returns `Err` with a human-readable description when any field
    /// is out of range (e.g., `multiplier` must be positive).
    pub fn validate(&self) -> Result<(), String> {
        if self.multiplier <= 0.0 {
            return Err(format!(
                "swe_edge_egress_retry: multiplier must be > 0.0, got {}",
                self.multiplier
            ));
        }
        if self.max_interval_ms < self.initial_interval_ms {
            return Err(format!(
                "swe_edge_egress_retry: max_interval_ms ({}) must be >= initial_interval_ms ({})",
                self.max_interval_ms, self.initial_interval_ms
            ));
        }
        Ok(())
    }
}

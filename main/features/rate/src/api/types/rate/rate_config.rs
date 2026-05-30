//! Client-side rate-limiter policy schema. Values live in
//! `config/application.toml`.

use serde::Deserialize;

use crate::api::error::RateError;

/// Rate-limiter (token-bucket) policy schema.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RateConfig {
    /// Sustained refill rate, tokens per second.
    pub tokens_per_second: u32,
    /// Bucket capacity (burst tolerance).
    pub burst_capacity: u32,
    /// Per-host bucketing (false = single global bucket).
    pub per_host: bool,
}

impl Default for RateConfig {
    fn default() -> Self {
        Self {
            tokens_per_second: 10,
            burst_capacity: 20,
            per_host: true,
        }
    }
}

impl swe_edge_configbuilder::ConfigSection for RateConfig {
    fn section_name() -> &'static str {
        "rate"
    }
}

impl RateConfig {
    /// Parse from TOML text.
    pub fn from_config(toml_text: &str) -> Result<Self, RateError> {
        toml::from_str(toml_text).map_err(|e| RateError::ParseFailed(e.to_string()))
    }
}

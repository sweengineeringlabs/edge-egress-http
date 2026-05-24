//! Client-side rate-limiter policy schema. Values live in
//! `config/application.toml`.

use serde::Deserialize;

use crate::api::error::Error;

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
            tokens_per_second = 50
            burst_capacity = 100
            per_host = false
        "#;
        let cfg = RateConfig::from_config(toml).unwrap();
        assert_eq!(cfg.tokens_per_second, 50);
        assert!(!cfg.per_host);
    }

    /// @covers: Default
    #[test]
    fn test_rate_config_default_has_positive_tokens_per_second() {
        let cfg = RateConfig::default();
        assert!(cfg.tokens_per_second >= 1);
    }

    /// @covers: ConfigSection::section_name
    #[test]
    fn test_rate_config_section_name_is_rate() {
        use swe_edge_configbuilder::ConfigSection as _;
        assert_eq!(RateConfig::section_name(), "rate");
    }
}

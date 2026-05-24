//! Circuit-breaker policy schema. Values live in
//! `config/application.toml`.

use serde::Deserialize;

use crate::api::error::Error;

/// Circuit-breaker policy schema.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BreakerConfig {
    /// Consecutive failures that trip the breaker open.
    pub failure_threshold: u32,
    /// Seconds to wait after opening before probing.
    pub half_open_after_seconds: u64,
    /// Consecutive probe successes required to close.
    pub reset_after_successes: u32,
    /// Response statuses counted as failures.
    pub failure_statuses: Vec<u16>,
}

impl Default for BreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            half_open_after_seconds: 30,
            reset_after_successes: 3,
            failure_statuses: vec![500, 502, 503, 504],
        }
    }
}

impl swe_edge_configbuilder::ConfigSection for BreakerConfig {
    fn section_name() -> &'static str {
        "breaker"
    }
}

impl BreakerConfig {
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
            failure_threshold = 10
            half_open_after_seconds = 60
            reset_after_successes = 5
            failure_statuses = [500, 503]
        "#;
        let cfg = BreakerConfig::from_config(toml).unwrap();
        assert_eq!(cfg.failure_threshold, 10);
    }

    /// @covers: Default
    #[test]
    fn test_breaker_config_default_has_positive_failure_threshold() {
        let cfg = BreakerConfig::default();
        assert!(cfg.failure_threshold >= 1);
    }

    /// @covers: ConfigSection::section_name
    #[test]
    fn test_breaker_config_section_name_is_breaker() {
        use swe_edge_configbuilder::ConfigSection as _;
        assert_eq!(BreakerConfig::section_name(), "breaker");
    }
}

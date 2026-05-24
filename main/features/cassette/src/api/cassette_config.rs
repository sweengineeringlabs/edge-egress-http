//! Cassette policy schema. Values live in `config/application.toml`.

use serde::Deserialize;

use crate::api::error::Error;

/// Cassette record/replay policy schema.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CassetteConfig {
    /// Operating mode: `"replay"` | `"record"` | `"auto"` | `"disabled"`.
    ///
    /// `"disabled"` passes every request straight through without touching
    /// any cassette file. Use this in production factory functions where
    /// record/replay is not wanted.
    pub mode: String,
    /// Cassette directory (relative to the test binary's manifest).
    pub cassette_dir: String,
    /// Request components included in the match key.
    pub match_on: Vec<String>,
    /// Headers to strip before writing the cassette.
    pub scrub_headers: Vec<String>,
    /// JSON paths inside the request body to zero out before
    /// hashing (e.g. `"request_id"`, `"metadata.trace_id"`).
    pub scrub_body_paths: Vec<String>,
}

impl Default for CassetteConfig {
    fn default() -> Self {
        Self {
            mode: "replay".into(),
            cassette_dir: "tests/cassettes".into(),
            match_on: vec!["method".into(), "url".into(), "body_hash".into()],
            scrub_headers: vec![
                "authorization".into(),
                "x-api-key".into(),
                "cookie".into(),
                "set-cookie".into(),
                "proxy-authorization".into(),
            ],
            scrub_body_paths: vec![],
        }
    }
}

impl swe_edge_configbuilder::ConfigSection for CassetteConfig {
    fn section_name() -> &'static str {
        "cassette"
    }
}

impl CassetteConfig {
    /// Parse from TOML text.
    pub fn from_config(toml_text: &str) -> Result<Self, Error> {
        toml::from_str(toml_text).map_err(|e| Error::ParseFailed(e.to_string()))
    }

    /// A config that passes every request straight through — no recording,
    /// no replay, no cassette file I/O. Use in production stacks where
    /// record/replay infrastructure is not wanted.
    pub fn disabled() -> Self {
        Self {
            mode: "disabled".into(),
            cassette_dir: String::new(),
            match_on: vec![],
            scrub_headers: vec![],
            scrub_body_paths: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: from_config
    #[test]
    fn test_from_config_parses_full_toml() {
        let toml = r#"
            mode = "auto"
            cassette_dir = "tests/fixtures"
            match_on = ["method", "url"]
            scrub_headers = ["authorization"]
            scrub_body_paths = []
        "#;
        let cfg = CassetteConfig::from_config(toml).unwrap();
        assert_eq!(cfg.mode, "auto");
    }

    /// @covers: Default
    #[test]
    fn test_cassette_config_default_scrubs_authorization_header() {
        let cfg = CassetteConfig::default();
        assert!(
            cfg.scrub_headers.iter().any(|h| h == "authorization"),
            "authorization must be scrubbed by default to prevent credential leaks"
        );
    }

    /// @covers: ConfigSection::section_name
    #[test]
    fn test_cassette_config_section_name_is_cassette() {
        use swe_edge_configbuilder::ConfigSection as _;
        assert_eq!(CassetteConfig::section_name(), "cassette");
    }
}

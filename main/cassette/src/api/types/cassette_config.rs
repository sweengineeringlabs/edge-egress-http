//! Cassette policy schema. Values live in `config/application.toml`.

use serde::Deserialize;

use crate::api::error::CassetteError;

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
    fn section_name() -> &'static str { // @allow: no_stub_fn_bodies
        "cassette"
    }
}

impl CassetteConfig {
    /// Parse from TOML text.
    pub fn from_config(toml_text: &str) -> Result<Self, CassetteError> {
        toml::from_str(toml_text).map_err(|e| CassetteError::ParseFailed(e.to_string()))
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

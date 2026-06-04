//! Fluent builder for [`CassetteConfig`].
//!
//! Rule 91: structs with 5+ fields require a builder. `CassetteConfig` has
//! 5 fields (`mode`, `cassette_dir`, `match_on`, `scrub_headers`, `scrub_body_paths`).

use crate::api::error::CassetteError;
use crate::api::types::cassette::cassette_config::CassetteConfig;

/// Fluent builder for [`CassetteConfig`].
///
/// All fields are optional — unset fields fall back to the `CassetteConfig::default()` values.
#[derive(Default)]
pub struct CassetteConfigBuilder {
    mode: Option<String>,
    cassette_dir: Option<String>,
    match_on: Option<Vec<String>>,
    scrub_headers: Option<Vec<String>>,
    scrub_body_paths: Option<Vec<String>>,
}

impl CassetteConfigBuilder {
    /// Create a new builder with all fields unset (defaults apply on `build`).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the operating mode: `"replay"` | `"record"` | `"auto"` | `"disabled"`.
    pub fn with_mode(mut self, mode: impl Into<String>) -> Self {
        self.mode = Some(mode.into());
        self
    }

    /// Set the cassette directory.
    pub fn with_cassette_dir(mut self, dir: impl Into<String>) -> Self {
        self.cassette_dir = Some(dir.into());
        self
    }

    /// Set the request components included in the match key.
    pub fn with_match_on(mut self, keys: Vec<String>) -> Self {
        self.match_on = Some(keys);
        self
    }

    /// Set the headers to strip before writing the cassette.
    pub fn with_scrub_headers(mut self, headers: Vec<String>) -> Self {
        self.scrub_headers = Some(headers);
        self
    }

    /// Set the JSON body paths to scrub before hashing.
    pub fn with_scrub_body_paths(mut self, paths: Vec<String>) -> Self {
        self.scrub_body_paths = Some(paths);
        self
    }

    /// Consume the builder and produce a [`CassetteConfig`].
    ///
    /// Returns an error if `mode` is set but is not one of the recognised values.
    pub fn build_config(self) -> Result<CassetteConfig, CassetteError> {
        let defaults = CassetteConfig::default();
        let mode = self.mode.unwrap_or(defaults.mode);
        match mode.as_str() {
            "replay" | "record" | "auto" | "disabled" => {}
            other => {
                return Err(CassetteError::ParseFailed(format!(
                    "unknown cassette mode '{other}'; expected replay|record|auto|disabled"
                )));
            }
        }
        Ok(CassetteConfig {
            mode,
            cassette_dir: self.cassette_dir.unwrap_or(defaults.cassette_dir),
            match_on: self.match_on.unwrap_or(defaults.match_on),
            scrub_headers: self.scrub_headers.unwrap_or(defaults.scrub_headers),
            scrub_body_paths: self.scrub_body_paths.unwrap_or(defaults.scrub_body_paths),
        })
    }
}

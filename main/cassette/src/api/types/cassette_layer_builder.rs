//! Fluent builder for [`CassetteLayer`].
//!
//! Rule 91: structs with 5+ fields require a builder.

use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::api::error::CassetteError;
use crate::api::types::cassette_config::CassetteConfig;
use crate::api::types::cassette_layer::CassetteLayer;

/// Fluent builder for [`CassetteLayer`].
///
/// Required: `cassette_name`. Optional: `config` (defaults to `CassetteConfig::default()`).
#[derive(Default)]
pub struct CassetteLayerBuilder {
    config: Option<CassetteConfig>,
    cassette_name: Option<String>,
}

impl CassetteLayerBuilder {
    /// Create a new builder with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the cassette configuration.
    pub fn with_config(mut self, config: CassetteConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Set the cassette name (used to derive the file path).
    pub fn with_cassette_name(mut self, name: impl Into<String>) -> Self {
        self.cassette_name = Some(name.into());
        self
    }

    /// Consume the builder and produce a [`CassetteLayer`].
    ///
    /// Returns an error if `cassette_name` was not set.
    pub fn build_layer(self) -> Result<CassetteLayer, CassetteError> {
        let config = self.config.unwrap_or_default();
        let cassette_name = self.cassette_name.ok_or_else(|| {
            CassetteError::ParseFailed("CassetteLayerBuilder: cassette_name is required".into())
        })?;
        let path = PathBuf::from(&config.cassette_dir).join(format!("{cassette_name}.yaml"));
        let fixtures = CassetteLayer::load_fixtures_from_disk(&path)?;
        Ok(CassetteLayer {
            config: Arc::new(config),
            cassette_path: path,
            fixtures: Arc::new(Mutex::new(fixtures)),
        })
    }
}

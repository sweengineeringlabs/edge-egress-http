//! HTTP rate SAF — factory methods on [`HttpRateSvc`].

use swe_edge_configbuilder::ConfigLoaderFactory;

use crate::api::error::RateError;
use crate::api::traits::Processor;
use crate::api::traits::Validator;
use crate::api::types::HttpRateSvc;
use crate::api::types::RateConfig;
use crate::api::types::RateLayer;
use crate::core::default_http_rate::DefaultHttpRate;

impl HttpRateSvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let builder = ConfigLoaderFactory::create_config_builder();
        builder
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Validate a [`RateConfig`] and build a [`RateLayer`] from it.
    ///
    /// Returns `Err` if the config fails validation (e.g. zero token rate).
    pub fn build_rate_layer(config: RateConfig) -> Result<RateLayer, RateError> {
        let processor = DefaultHttpRate::new(config.clone());
        processor
            .validate()
            .map_err(|e| RateError::ParseFailed(e))?;
        let _ = processor.describe(); // exercise the Processor contract
        let layer = RateLayer::new(config);
        Ok(layer)
    }
}

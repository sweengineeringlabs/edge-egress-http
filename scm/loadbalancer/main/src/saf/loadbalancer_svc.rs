//! HTTP loadbalancer SAF — factory methods on [`LoadbalancerSvc`].

use crate::api::error::LoadbalancerMiddlewareError;
use crate::api::traits::{Processor, Validator};
use crate::api::types::{LoadbalancerConfig, LoadbalancerLayer, LoadbalancerSvc};
use crate::core::middleware::DefaultLoadbalancerMiddleware;

impl LoadbalancerSvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }

    /// Validate a [`LoadbalancerConfig`] and build a [`LoadbalancerLayer`] from it.
    ///
    /// Returns `Err` if the config fails validation (e.g. empty backend list,
    /// zero-weight backend).
    ///
    /// # Errors
    ///
    /// - [`LoadbalancerMiddlewareError::InvalidConfig`] — validation failed.
    /// - [`LoadbalancerMiddlewareError::PoolError`] — pool construction failed.
    pub fn build_layer(
        config: LoadbalancerConfig,
    ) -> Result<LoadbalancerLayer, LoadbalancerMiddlewareError> {
        let processor = DefaultLoadbalancerMiddleware::new(config.clone());
        processor
            .validate()
            .map_err(LoadbalancerMiddlewareError::InvalidConfig)?;
        let _ = processor.describe(); // exercise the Processor contract
        LoadbalancerLayer::new(config)
    }

    /// Validate a [`LoadbalancerConfig`] without constructing a layer.
    pub fn validate_config(config: &LoadbalancerConfig) -> Result<(), String> {
        DefaultLoadbalancerMiddleware::new(config.clone()).validate()
    }
}

/// Validate a [`LoadbalancerConfig`] and build a [`LoadbalancerLayer`] from it.
///
/// # Errors
///
/// - [`LoadbalancerMiddlewareError::InvalidConfig`] — validation failed.
/// - [`LoadbalancerMiddlewareError::PoolError`] — pool construction failed.
pub fn build_loadbalancer_layer(
    config: LoadbalancerConfig,
) -> Result<LoadbalancerLayer, LoadbalancerMiddlewareError> {
    LoadbalancerSvc::build_layer(config)
}

/// Validate a [`LoadbalancerConfig`] without constructing a layer.
///
/// Returns `Ok(())` when the config is well-formed.
pub fn validate_loadbalancer_config(config: &LoadbalancerConfig) -> Result<(), String> {
    LoadbalancerSvc::validate_config(config)
}

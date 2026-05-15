//! Public builder entry point.

use crate::api::error::Error;
use crate::api::retry_config::RetryConfig;
use crate::api::retry_layer::RetryLayer;

/// Start configuring the retry middleware with the SWE baseline
/// loaded from the crate-shipped `config/application.toml`.
pub fn builder() -> Result<ApplicationConfigBuilder, Error> {
    let cfg = RetryConfig::swe_default()?;
    Ok(ApplicationConfigBuilder::with_config(cfg))
}

pub use crate::api::builder::ApplicationConfigBuilder;

impl ApplicationConfigBuilder {
    /// Construct from a caller-supplied config.
    pub fn with_config(config: RetryConfig) -> Self {
        Self { config }
    }

    /// Borrow the current policy.
    pub fn config(&self) -> &RetryConfig {
        &self.config
    }

    /// Finalize into the [`RetryLayer`]. Takes config by value;
    /// the resulting middleware holds an `Arc<RetryConfig>` so
    /// it can be cloned cheaply across the middleware chain.
    pub fn build(self) -> Result<RetryLayer, Error> {
        Ok(RetryLayer::new(self.config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: builder
    #[test]
    fn test_builder_loads_swe_default() {
        let b = builder().expect("baseline parses");
        assert!(b.config().max_retries >= 1);
    }

    /// @covers: ApplicationConfigBuilder::build
    #[test]
    fn test_build_returns_retry_layer() {
        let layer = builder().expect("baseline").build().expect("build ok");
        let s = format!("{layer:?}");
        assert!(s.contains("RetryLayer"));
        assert!(s.contains("max_retries"));
    }
}

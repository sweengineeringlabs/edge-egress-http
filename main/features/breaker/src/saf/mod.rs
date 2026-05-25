//! SAF layer — public facade.

mod builder;

pub use crate::api::error::BreakerError;
pub use crate::api::types::breaker_config::BreakerConfig;
pub use crate::api::types::breaker_layer::BreakerLayer;
pub use builder::{build_breaker_layer, create_config_builder};

/// Error type alias for compatibility.
pub type Error = BreakerError;

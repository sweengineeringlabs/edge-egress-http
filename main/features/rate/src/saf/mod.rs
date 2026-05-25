//! SAF layer — public facade.

mod builder;

pub use crate::api::error::RateError;
pub use crate::api::types::rate_config::RateConfig;
pub use crate::api::types::rate_layer::RateLayer;
pub use builder::{build_rate_layer, create_config_builder};

/// Error type alias for compatibility.
pub type Error = RateError;

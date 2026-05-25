//! SAF layer — public facade.

mod builder;

pub use crate::api::error::RetryError;
pub use crate::api::types::retry_config::RetryConfig;
pub use crate::api::types::retry_layer::RetryLayer;
pub use builder::{build_retry_layer, create_config_builder};

/// Error type alias for compatibility.
pub type Error = RetryError;

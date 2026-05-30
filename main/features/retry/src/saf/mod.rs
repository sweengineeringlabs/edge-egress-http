//! SAF layer — public facade.

mod retry_svc;

pub use crate::api::types::HttpRetrySvc;

pub use crate::api::error::RetryError;
pub use crate::api::types::retry_config::RetryConfig;
pub use crate::api::types::retry_layer::RetryLayer;

/// Error type alias for compatibility.
pub type Error = RetryError;

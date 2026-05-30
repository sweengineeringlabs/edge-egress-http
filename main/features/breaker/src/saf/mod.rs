//! SAF layer — public facade.

mod breaker_svc;

pub use crate::api::types::HttpBreakerSvc;

pub use crate::api::error::BreakerError;
pub use crate::api::types::breaker_config::BreakerConfig;
pub use crate::api::types::breaker_layer::BreakerLayer;

/// Error type alias for compatibility.
pub type Error = BreakerError;

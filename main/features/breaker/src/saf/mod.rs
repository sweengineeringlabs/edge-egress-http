//! SAF layer — public facade.

mod breaker_svc;

pub use crate::api::types::HttpBreakerSvc;

pub use crate::api::error::BreakerError;
pub use crate::api::error::Error;
pub use crate::api::types::breaker::config::BreakerConfig;
pub use crate::api::types::breaker::layer::BreakerLayer;

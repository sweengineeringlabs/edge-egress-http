//! SAF layer — public facade.

mod breaker_svc;

pub use crate::api::types::HttpBreakerSvc;

pub use crate::api::error::BreakerError;
pub use crate::api::error::Error;
pub use crate::api::types::breaker::admission::Admission;
pub use crate::api::types::breaker::breaker_config::BreakerConfig;
pub use crate::api::types::breaker::breaker_layer::BreakerLayer;
pub use crate::api::types::breaker::outcome::Outcome;

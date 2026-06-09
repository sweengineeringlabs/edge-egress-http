//! SAF layer — public facade.

mod breaker_svc;

pub use breaker_svc::get_failure_threshold;

pub use crate::api::types::HttpBreakerSvc;

pub use crate::api::error::BreakerError;
pub use crate::api::error::Error;
pub use crate::api::types::admission::Admission;
pub use crate::api::types::breaker_config::BreakerConfig;
pub use crate::api::types::breaker_layer::BreakerLayer;
pub use crate::api::types::outcome::Outcome;

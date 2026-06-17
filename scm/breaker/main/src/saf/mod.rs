//! SAF layer — public facade.

mod breaker;
mod circuit_breaker_node_svc;
mod host_breaker_svc;
mod http_breaker_svc;
mod processor_svc;
mod state_svc;
mod validator_svc;

pub use breaker::get_failure_threshold;

pub use crate::api::types::HttpBreakerSvc;

pub use crate::api::error::BreakerError;
pub use crate::api::error::Error;
pub use crate::api::types::admission::Admission;
pub use crate::api::types::breaker_config::BreakerConfig;
pub use crate::api::types::breaker_layer::BreakerLayer;
pub use crate::api::types::outcome::Outcome;

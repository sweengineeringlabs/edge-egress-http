//! SAF layer — public facade.

mod breaker;
mod circuit_breaker_node_svc;
mod host_breaker_svc;
mod http_breaker_svc;
mod processor_svc;
mod state_svc;
mod validator_svc;

pub(crate) use breaker::get_failure_threshold;

pub(crate) use crate::api::types::HttpBreakerSvc;

pub(crate) use crate::api::error::BreakerError;
pub(crate) use crate::api::error::Error;
pub(crate) use crate::api::types::admission::Admission;
pub(crate) use crate::api::types::breaker_config::BreakerConfig;
pub(crate) use crate::api::types::breaker_layer::BreakerLayer;
pub(crate) use crate::api::types::outcome::Outcome;

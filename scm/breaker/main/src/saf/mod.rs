//! SAF layer — public facade.

mod breaker;
mod circuit_breaker_node_svc;
mod host_breaker_svc;
mod http_breaker_svc;
mod processor_svc;
mod state_svc;
mod validator_svc;

pub use breaker::get_failure_threshold;

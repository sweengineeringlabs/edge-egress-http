//! Value objects for the breaker API.
pub(crate) mod breaker_config;
pub(crate) mod breaker_layer;

pub mod breaker_svc;
pub use breaker_svc::HttpBreakerSvc;

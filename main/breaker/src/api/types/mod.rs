//! Value objects for the breaker API.

pub(crate) mod application_config_builder;
pub(crate) mod breaker;

pub use breaker::http_breaker_svc::HttpBreakerSvc;

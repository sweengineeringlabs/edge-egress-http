//! Value objects for the breaker API.

pub(crate) mod application_config_builder;

pub(crate) mod admission;
pub(crate) mod breaker_config;
pub(crate) mod breaker_layer;
pub(crate) mod http_breaker_svc;
pub(crate) mod outcome;
pub(crate) mod state;

pub use http_breaker_svc::HttpBreakerSvc;

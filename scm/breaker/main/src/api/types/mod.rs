//! Value objects for the breaker API.

pub(crate) mod application_config_builder;

pub(crate) mod admission;
pub mod breaker;
pub(crate) mod http_breaker_svc;
pub(crate) mod outcome;
pub(crate) mod state;

// Compatibility re-exports: keep existing `crate::api::types::breaker_config::…`
// and `crate::api::types::breaker_layer::…` paths working after the R112 move.
pub(crate) use breaker::breaker_config;
pub(crate) use breaker::breaker_layer;

pub use http_breaker_svc::HttpBreakerSvc;

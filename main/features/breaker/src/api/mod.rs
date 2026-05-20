//! API layer — public schema + trait contracts + public types.
pub mod application_config_builder;
pub(crate) mod breaker_config;
pub(crate) mod breaker_layer;
pub(crate) mod breaker_state;
pub mod builder;
pub(crate) mod error;
pub(crate) mod host_breaker;
pub(crate) mod traits;

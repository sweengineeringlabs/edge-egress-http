//! SAF layer — public facade.

mod rate_svc;

pub use crate::api::types::HttpRateSvc;

pub use crate::api::error::RateError;
pub use crate::api::types::rate_config::RateConfig;
pub use crate::api::types::rate_layer::RateLayer;

/// Error type alias for compatibility.
pub type Error = RateError;

//! SAF layer — public facade.

mod rate_svc;

pub use crate::api::types::HttpRateSvc;

pub use crate::api::error::Error;
pub use crate::api::error::RateError;
pub use crate::api::types::RateConfig;
pub use crate::api::types::RateLayer;

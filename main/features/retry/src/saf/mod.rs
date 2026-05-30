//! SAF layer — public facade.

mod retry_svc;

pub use crate::api::error::RetryError;
pub use crate::api::types::retry::HttpRetrySvc;
pub use crate::api::types::retry::RetryConfig;
pub use crate::api::types::retry::RetryLayer;

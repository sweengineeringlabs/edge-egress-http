//! SAF layer — public facade.

mod retry_svc;

pub use crate::api::error::RetryError;
pub use crate::api::types::HttpRetrySvc;
pub use crate::api::types::RetryConfig;
pub use crate::api::types::RetryConfigBuilder;
pub use crate::api::types::RetryLayer;

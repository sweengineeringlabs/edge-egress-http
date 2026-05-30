//! SAF layer — public facade.

mod cache_svc;

pub use crate::api::types::HttpCacheSvc;

pub use crate::api::error::CacheError;
pub use crate::api::types::cache_config::CacheConfig;
pub use crate::api::types::cache_layer::CacheLayer;

/// Error type alias for compatibility.
pub type Error = CacheError;

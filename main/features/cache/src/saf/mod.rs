//! SAF layer — public facade.

mod builder;

pub use crate::api::error::CacheError;
pub use crate::api::types::cache_config::CacheConfig;
pub use crate::api::types::cache_layer::CacheLayer;
pub use builder::{build_cache_layer, create_config_builder};

/// Error type alias for compatibility.
pub type Error = CacheError;

//! SAF layer — public facade.

mod builder;

pub use crate::api::cache_config::CacheConfig;
pub use crate::api::cache_layer::CacheLayer;
pub use crate::api::error::Error;
pub use builder::{build_cache_layer, create_config_builder};

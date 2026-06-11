//! SAF layer — public facade.

mod cache;
mod http_cache_svc;
mod processor_svc;
mod request_snapshot_svc;
mod ttl_decision_svc;
mod validator_svc;
mod vary_directive_svc;

pub use crate::api::types::CachedEntry;
pub use crate::api::types::CachedEntryBuilder;
pub use crate::api::types::HttpCacheSvc;

pub use crate::api::error::CacheError;
pub use crate::api::error::Error;
pub use crate::api::types::cache_config::CacheConfig;
pub use crate::api::types::cache_layer::CacheLayer;

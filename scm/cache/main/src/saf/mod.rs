//! SAF layer — public facade.

mod cache;
mod http_cache_svc;
mod processor_svc;
mod request_snapshot_svc;
mod ttl_decision_svc;
mod validator_svc;
mod vary_directive_svc;

pub(crate) use crate::api::types::CachedEntry;
pub(crate) use crate::api::types::CachedEntryBuilder;
pub(crate) use crate::api::types::HttpCacheSvc;

pub(crate) use crate::api::error::CacheError;
pub(crate) use crate::api::error::Error;
pub(crate) use crate::api::types::cache_config::CacheConfig;
pub(crate) use crate::api::types::cache_layer::CacheLayer;

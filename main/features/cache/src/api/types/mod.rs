//! Value objects for the cache API.
pub(crate) mod cache_config;
pub(crate) mod cache_layer;

pub mod cache_svc;
pub use cache_svc::HttpCacheSvc;

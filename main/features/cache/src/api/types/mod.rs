//! Value objects for the cache API.

pub mod cache;

pub mod application_config_builder;
pub use application_config_builder::ApplicationConfigBuilder;

// Re-export canonical names at the types/ level for backward compatibility.
pub use cache::cache_config::CacheConfig;
pub use cache::cache_layer::CacheLayer;
pub use cache::http_cache_svc::HttpCacheSvc;

// Retain legacy module paths for any existing use-sites inside this crate.
pub(crate) mod cache_config {
    pub use super::cache::cache_config::CacheConfig;
}
pub(crate) mod cache_layer {
    pub use super::cache::cache_layer::CacheLayer;
}
pub(crate) mod cache_svc {
    pub use super::cache::http_cache_svc::HttpCacheSvc;
}
pub mod cached_entry;
pub use cached_entry::CachedEntry;
pub mod entry;
pub use entry::CachedEntryBuilder;

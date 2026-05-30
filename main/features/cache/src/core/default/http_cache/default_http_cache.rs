//! Default impl of [`HttpCache`](crate::api::http_cache::HttpCache).

use crate::api::http_cache::HttpCache;
use crate::api::types::cache_config::CacheConfig;

/// Default HttpCache implementation. `pub(crate)` — consumers
/// never touch this type directly; they go through `saf::builder`.
#[derive(Debug)]
pub(crate) struct DefaultHttpCache {
    config: CacheConfig,
}

impl DefaultHttpCache {
    /// Construct from a resolved config.
    pub(crate) fn new(config: CacheConfig) -> Self {
        Self { config }
    }
}

impl HttpCache for DefaultHttpCache {
    fn describe(&self) -> &'static str {
        "swe_edge_egress_cache"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: DefaultHttpCache::new
    #[test]
    fn test_new_constructs_and_stores_config() {
        let cfg = CacheConfig::default();
        let d = DefaultHttpCache::new(cfg);
        let dbg = format!("{d:?}");
        assert!(dbg.contains("DefaultHttpCache"), "debug output: {dbg}");
    }

    /// @covers: DefaultHttpCache::describe
    #[test]
    fn test_describe_returns_crate_name() {
        let cfg = CacheConfig::default();
        let d = DefaultHttpCache::new(cfg);
        assert_eq!(d.describe(), "swe_edge_egress_cache");
    }
}

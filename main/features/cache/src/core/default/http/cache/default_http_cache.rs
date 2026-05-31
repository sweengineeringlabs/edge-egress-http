//! Default impl of [`HttpCache`](crate::api::traits::HttpCache).

use crate::api::traits::HttpCache;
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
        const LABEL: &str = "http-cache";
        LABEL
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_constructs_and_stores_config() {
        let cfg = CacheConfig::default();
        let d = DefaultHttpCache::new(cfg);
        let dbg = format!("{d:?}");
        assert!(dbg.contains("DefaultHttpCache"), "debug output: {dbg}");
    }

    /// @covers: describe
    #[test]
    fn test_describe_returns_crate_name() {
        let cfg = CacheConfig::default();
        let d = DefaultHttpCache::new(cfg);
        assert_eq!(d.describe(), "http-cache");
    }
}

//! End-to-end tests for the swe_edge_egress_cache SAF builder surface.

use swe_edge_egress_cache::{ApplicationConfigBuilder, CacheConfig, CacheLayer};

fn make_cfg() -> CacheConfig {
    CacheConfig {
        default_ttl_seconds: 300,
        max_entries: 100,
        respect_cache_control: true,
        cache_private: false,
    }
}

/// @covers: builder
#[test]
fn test_e2e_builder() {
    let layer: CacheLayer = swe_edge_egress_cache::builder()
        .expect("builder() must succeed")
        .build()
        .expect("build() must succeed");
    let s = format!("{layer:?}");
    assert!(
        s.contains("CacheLayer"),
        "e2e: Debug must contain 'CacheLayer': {s}"
    );
}

/// @covers: ApplicationConfigBuilder::with_config
#[test]
fn test_e2e_with_config() {
    let b = ApplicationConfigBuilder::with_config(make_cfg());
    assert_eq!(b.config().default_ttl_seconds, 300);
    b.build().expect("e2e with_config build must succeed");
}

/// @covers: ApplicationConfigBuilder::config
#[test]
fn test_e2e_config() {
    let b = ApplicationConfigBuilder::with_config(make_cfg());
    assert_eq!(b.config().max_entries, 100);
    assert!(b.config().respect_cache_control);
}

/// @covers: ApplicationConfigBuilder::build
#[test]
fn test_e2e_build() {
    let cfg = CacheConfig {
        default_ttl_seconds: 60,
        max_entries: 50,
        respect_cache_control: false,
        cache_private: true,
    };
    let layer = ApplicationConfigBuilder::with_config(cfg)
        .build()
        .expect("e2e build must succeed");
    assert!(!format!("{layer:?}").is_empty());
}

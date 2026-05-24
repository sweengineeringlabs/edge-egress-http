//! End-to-end tests for the swe_edge_egress_cache SAF builder surface.

use swe_edge_egress_cache::{build_cache_layer, create_config_builder, CacheConfig, CacheLayer};

fn make_cfg() -> CacheConfig {
    CacheConfig {
        default_ttl_seconds: 300,
        max_entries: 100,
        respect_cache_control: true,
        cache_private: false,
    }
}

/// @covers: build_cache_layer with default config
#[test]
fn test_e2e_builder() {
    let layer: CacheLayer = build_cache_layer(CacheConfig::default())
        .expect("build must succeed");
    let s = format!("{layer:?}");
    assert!(
        s.contains("CacheLayer"),
        "e2e: Debug must contain 'CacheLayer': {s}"
    );
}

/// @covers: build_cache_layer stores config fields correctly
#[test]
fn test_e2e_with_config() {
    let cfg = make_cfg();
    assert_eq!(cfg.default_ttl_seconds, 300);
    build_cache_layer(cfg).expect("e2e with_config build must succeed");
}

/// @covers: CacheConfig fields are accessible directly
#[test]
fn test_e2e_config() {
    let cfg = make_cfg();
    assert_eq!(cfg.max_entries, 100);
    assert!(cfg.respect_cache_control);
}

/// @covers: build_cache_layer with custom config
#[test]
fn test_e2e_build() {
    let cfg = CacheConfig {
        default_ttl_seconds: 60,
        max_entries: 50,
        respect_cache_control: false,
        cache_private: true,
    };
    let layer = build_cache_layer(cfg)
        .expect("e2e build must succeed");
    assert!(!format!("{layer:?}").is_empty());
}

/// @covers: create_config_builder returns a working Loader
#[test]
fn test_e2e_create_config_builder_returns_loader() {
    use swe_edge_configbuilder::ConfigBuilder as _;
    let _loader = create_config_builder().build_loader();
}

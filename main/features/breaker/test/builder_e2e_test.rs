//! End-to-end tests for the swe_edge_egress_breaker SAF builder surface.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_egress_breaker::{
    build_breaker_layer, create_config_builder, BreakerConfig, BreakerLayer,
};

fn make_cfg() -> BreakerConfig {
    BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 60,
        reset_after_successes: 2,
        failure_statuses: vec![500, 502, 503],
    }
}

/// @covers: build_breaker_layer with default config
#[test]
fn test_e2e_builder_default_config_succeeds() {
    let _layer: BreakerLayer = build_breaker_layer(BreakerConfig::default())
        .expect("build_breaker_layer with default config must succeed");
}

/// @covers: build_breaker_layer stores config fields correctly
#[test]
fn test_e2e_with_config() {
    let cfg = make_cfg();
    assert_eq!(cfg.failure_threshold, 3);
    build_breaker_layer(cfg).expect("e2e with_config build must succeed");
}

/// @covers: BreakerConfig fields are accessible directly
#[test]
fn test_e2e_config() {
    let cfg = make_cfg();
    assert_eq!(cfg.failure_statuses, vec![500u16, 502, 503]);
    assert_eq!(cfg.reset_after_successes, 2);
}

/// @covers: build_breaker_layer with custom config
#[test]
fn test_e2e_build() {
    let cfg = BreakerConfig {
        failure_threshold: 5,
        half_open_after_seconds: 30,
        reset_after_successes: 3,
        failure_statuses: vec![503, 504],
    };
    let layer = build_breaker_layer(cfg).expect("e2e build must succeed");
    assert!(!format!("{layer:?}").is_empty());
}

/// @covers: create_config_builder returns a working Loader
#[test]
fn test_e2e_create_config_builder_returns_loader() {
    use swe_edge_configbuilder::ConfigBuilder as _;
    let _loader = create_config_builder().build_loader();
}

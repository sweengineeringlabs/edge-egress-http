//! End-to-end tests for the swe_edge_egress_rate SAF builder surface.

use swe_edge_egress_rate::{ApplicationConfigBuilder, RateConfig, RateLayer};

fn make_cfg() -> RateConfig {
    RateConfig {
        tokens_per_second: 10,
        burst_capacity: 20,
        per_host: false,
    }
}

/// @covers: builder
#[test]
fn test_e2e_builder() {
    let layer: RateLayer = swe_edge_egress_rate::builder()
        .expect("builder() must succeed")
        .build()
        .expect("build() must succeed");
    assert!(format!("{layer:?}").contains("RateLayer"));
}

/// @covers: ApplicationConfigBuilder::with_config
#[test]
fn test_e2e_with_config() {
    let b = ApplicationConfigBuilder::with_config(make_cfg());
    assert_eq!(b.config().tokens_per_second, 10);
    b.build().expect("e2e with_config build must succeed");
}

/// @covers: ApplicationConfigBuilder::config
#[test]
fn test_e2e_config() {
    let b = ApplicationConfigBuilder::with_config(make_cfg());
    assert_eq!(b.config().burst_capacity, 20);
    assert!(!b.config().per_host);
}

/// @covers: ApplicationConfigBuilder::build
#[test]
fn test_e2e_build() {
    let cfg = RateConfig {
        tokens_per_second: 100,
        burst_capacity: 500,
        per_host: true,
    };
    let layer = ApplicationConfigBuilder::with_config(cfg)
        .build()
        .expect("e2e build must succeed");
    assert!(!format!("{layer:?}").is_empty());
}

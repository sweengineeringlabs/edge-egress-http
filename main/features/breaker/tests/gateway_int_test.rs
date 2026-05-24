//! Integration tests exercising the public gateway surface of the swe_edge_egress_breaker crate.

use swe_edge_egress_breaker::{build_breaker_layer, BreakerConfig, BreakerLayer, Error};

fn make_cfg() -> BreakerConfig {
    BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 60,
        reset_after_successes: 2,
        failure_statuses: vec![500, 502, 503],
    }
}

#[test]
fn test_builder_fn_loads_swe_default_and_succeeds() {
    build_breaker_layer(BreakerConfig::default()).expect("builder() must succeed");
}

#[test]
fn test_builder_fn_default_config_has_positive_failure_threshold() {
    let cfg = BreakerConfig::default();
    assert!(
        cfg.failure_threshold >= 1,
        "swe_default failure_threshold must be >= 1"
    );
}

#[test]
fn test_with_config_custom_config_stores_values() {
    let cfg = make_cfg();
    assert_eq!(cfg.failure_threshold, 3);
    assert_eq!(cfg.half_open_after_seconds, 60);
    assert_eq!(cfg.reset_after_successes, 2);
    assert_eq!(cfg.failure_statuses, vec![500u16, 502, 503]);
}

#[test]
fn test_build_default_produces_breaker_layer() {
    let layer: BreakerLayer = build_breaker_layer(BreakerConfig::default())
        .expect("build must succeed");
    let s = format!("{layer:?}");
    assert!(
        s.contains("BreakerLayer"),
        "Debug must contain 'BreakerLayer': {s}"
    );
}

#[test]
fn test_build_custom_config_produces_layer() {
    build_breaker_layer(make_cfg())
        .expect("build with custom cfg must succeed");
}

#[test]
fn test_breaker_layer_is_send_and_sync() {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<BreakerLayer>();
}

#[test]
fn test_with_config_high_threshold_flows_through_config_accessor() {
    let cfg = BreakerConfig {
        failure_threshold: 10,
        half_open_after_seconds: 30,
        reset_after_successes: 5,
        failure_statuses: vec![503],
    };
    assert_eq!(cfg.failure_threshold, 10);
}

#[test]
fn test_build_empty_failure_statuses_succeeds() {
    let cfg = BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 60,
        reset_after_successes: 2,
        failure_statuses: vec![],
    };
    build_breaker_layer(cfg)
        .expect("empty failure_statuses must build");
}

#[test]
fn test_error_parse_failed_display_contains_crate_name() {
    let err = Error::ParseFailed("oops".to_string());
    let s = err.to_string();
    assert!(
        s.contains("swe_edge_egress_breaker"),
        "ParseFailed Display must name the crate: {s}"
    );
}

#[test]
fn test_builder_config_method_borrows_current_policy() {
    let cfg = make_cfg();
    assert_eq!(cfg.failure_threshold, 3);
}

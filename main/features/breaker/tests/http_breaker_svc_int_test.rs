//! Integration tests for `api/types/breaker/http_breaker_svc.rs`.
//! @covers: src/api/types/breaker/http_breaker_svc.rs

use swe_edge_egress_breaker::{BreakerConfig, HttpBreakerSvc};

/// @covers: HttpBreakerSvc
/// Confirms the factory method produces a valid `BreakerLayer` from a
/// non-default config.
#[test]
fn breaker_struct_http_breaker_svc_build_layer_custom_config_int_test() {
    let cfg = BreakerConfig {
        failure_threshold: 5,
        half_open_after_seconds: 60,
        reset_after_successes: 3,
        failure_statuses: vec![500, 502, 503],
    };
    let result = HttpBreakerSvc::build_breaker_layer(cfg);
    assert!(
        result.is_ok(),
        "build_breaker_layer must succeed with a custom config"
    );
}

/// @covers: HttpBreakerSvc
/// Confirms `create_config_builder` returns a usable builder.
#[test]
fn breaker_struct_http_breaker_svc_create_config_builder_int_test() {
    let builder = HttpBreakerSvc::create_config_builder();
    // Building a loader from the seeded builder must not panic.
    let _loader = builder.build_loader();
}

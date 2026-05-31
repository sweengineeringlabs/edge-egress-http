//! Integration tests for the `BreakerMetrics` trait contract.

use swe_edge_egress_breaker::{BreakerMetrics, HttpBreakerSvc};

/// @covers: BreakerMetrics — trait is object-safe
#[test]
fn breaker_trait_metrics_is_object_safe_int_test() {
    fn _assert(_: &dyn BreakerMetrics) {}
}

/// @covers: HttpBreakerSvc::create_config_builder — dep coverage for swe-edge-configbuilder
#[test]
fn breaker_struct_http_breaker_svc_create_config_builder_returns_seeded_builder_int_test() {
    let builder = HttpBreakerSvc::create_config_builder();
    assert!(
        !builder.name().is_empty(),
        "builder must be seeded with crate name"
    );
}

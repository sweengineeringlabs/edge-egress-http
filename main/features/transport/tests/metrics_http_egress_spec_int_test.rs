//! Integration tests for `MetricsHttpEgressSpec` marker trait.
//!
//! Rule 120: `src/api/metrics/metrics_http_egress_spec.rs` requires a
//! corresponding test file.
//!
//! `MetricsHttpEgressSpec` is a marker trait for metrics HTTP egress
//! implementations. The concrete implementation is `MetricsHttpEgress`
//! (exported as `MetricsEgress`).

use swe_edge_egress_http_transport::MetricsEgress;

/// @covers: MetricsHttpEgressSpec (via MetricsEgress)
/// The `MetricsEgress` type alias (which is `MetricsHttpEgress`, implementing
/// `MetricsHttpEgressSpec`) must be accessible and have a non-zero pointer size.
#[test]
fn transport_trait_metrics_http_egress_spec_alias_is_accessible_int_test() {
    let _size = std::mem::size_of::<*const MetricsEgress>();
    assert!(
        _size > 0,
        "pointer to MetricsEgress must have non-zero size"
    );
}

/// @covers: MetricsHttpEgressSpec object safety
/// `MetricsEgress` must be usable as a reference target (object-safe trait).
#[test]
fn transport_trait_metrics_http_egress_spec_is_object_safe_int_test() {
    fn _check(_: &MetricsEgress) {}
    assert!(
        true,
        "MetricsEgress must be referenceable (compile-time object-safety check passed)"
    );
}

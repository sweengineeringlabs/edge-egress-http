//! Integration tests for `MetricsHttpEgress`.

use swe_edge_egress_http_transport::MetricsEgress;

#[test]
fn test_metrics_http_egress_type_is_object_safe() {
    fn _check(_: &MetricsEgress) {}
}

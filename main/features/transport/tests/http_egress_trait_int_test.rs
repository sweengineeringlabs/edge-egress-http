//! Integration tests for `HttpEgress`.

use swe_edge_egress_http_transport::HttpEgress;

#[test]
fn test_http_egress_trait_is_object_safe() {
    fn _assert_object_safe(_: &dyn HttpEgress) {}
}

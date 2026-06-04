//! Integration tests for `http_retry_svc` in `swe-edge-egress-retry`.

use swe_edge_egress_retry::HttpRetrySvc;

/// @covers: HttpRetrySvc
#[test]
fn test_http_retry_svc_is_accessible() {
    let _exists = core::marker::PhantomData::<HttpRetrySvc>;
}

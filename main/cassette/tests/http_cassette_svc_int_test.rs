//! Integration tests for `http_cassette_svc` in `swe-edge-egress-cassette`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_egress_cassette::HttpCassetteSvc;

/// @covers: HttpCassetteSvc
/// Confirms `HttpCassetteSvc` is publicly accessible in the crate.
#[test]
fn test_http_cassette_svc_is_accessible() {
    let _exists = core::marker::PhantomData::<HttpCassetteSvc>;
}

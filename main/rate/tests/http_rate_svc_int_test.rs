//! Integration tests for `http_rate_svc` in `swe-edge-egress-rate`.

/// @covers: HttpRateSvc
#[test]
fn test_http_rate_svc_is_accessible() {
    // Naming the type from an external crate proves it is re-exported as part
    // of the public API. If this compiles, the re-export exists.
    let _exists = core::marker::PhantomData::<swe_edge_egress_rate::HttpRateSvc>;
}

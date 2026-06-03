//! Integration tests for `http_cache_svc` in `swe-edge-egress-cache`.

use swe_edge_egress_cache::HttpCacheSvc;

/// @covers: HttpCacheSvc
#[test]
fn test_http_cache_svc_is_accessible() {
    // Verify HttpCacheSvc is part of the public API by constructing the type
    // directly — fails to compile if the re-export is removed.
    let _exists = HttpCacheSvc;
    let _ = _exists;
}

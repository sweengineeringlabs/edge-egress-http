//! Integration tests for `cache_error` in `swe-edge-egress-cache`.

use swe_edge_egress_cache::CacheError;

/// @covers: CacheError
#[test]
fn test_cache_error_is_accessible() {
    // Verify CacheError is part of the public API by instantiating a PhantomData
    // marker — if the type is not exported this file will not compile.
    let _exists = core::marker::PhantomData::<CacheError>;
}

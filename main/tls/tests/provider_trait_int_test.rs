//! Integration tests for `provider_trait` in `swe-edge-egress-tls`.

use swe_edge_egress_tls::Provider;

/// @covers: Provider
/// Proves `Provider` is object-safe and accessible from the crate root.
/// A removed re-export or a broken object-safety bound causes this to fail
/// to compile.
#[test]
fn test_provider_trait_is_accessible() {
    let _ = core::marker::PhantomData::<dyn Provider>;
}

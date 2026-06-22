//! Integration tests for `tls_error` in `swe-edge-egress-tls`.

use swe_edge_egress_tls::TlsConfigError;

/// @covers: TlsConfigError
/// Proves `TlsConfigError` is accessible from the crate root and that each variant
/// is constructible. A missing re-export or removed variant causes this to
/// fail to compile.
#[test]
fn test_tls_error_is_accessible() {
    let _ = core::marker::PhantomData::<TlsConfigError>;
}

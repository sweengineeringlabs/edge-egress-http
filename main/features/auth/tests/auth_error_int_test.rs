//! Integration tests for `auth_error` in `swe-edge-egress-auth`.

use core::marker::PhantomData;
use swe_edge_egress_auth::AuthError;

/// @covers: AuthError
/// Fails to compile if `AuthError` is removed from the crate's public surface.
#[test]
fn test_auth_error_type_is_accessible_from_public_api() {
    let _exists = PhantomData::<AuthError>;
}

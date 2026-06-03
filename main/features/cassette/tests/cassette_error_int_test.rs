//! Integration tests for `cassette_error` in `swe-edge-egress-cassette`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_egress_cassette::CassetteError;

/// @covers: CassetteError
/// Confirms `CassetteError` is a publicly accessible type.
#[test]
fn test_cassette_error_is_defined() {
    let _exists = core::marker::PhantomData::<CassetteError>;
}

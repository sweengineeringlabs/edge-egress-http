//! Integration tests for `Result` type alias.
//!
//! Rule 120: `src/api/error/result.rs` requires a corresponding test file.

use swe_edge_egress_oauth::{OAuthError, Result};

/// @covers: Result type alias
/// The `Result` alias must be usable as a return type and must propagate errors.
#[test]
fn oauth_struct_o_auth_result_ok_variant_int_test() {
    let ok: Result<u32> = Ok(42u32);
    assert_eq!(ok.unwrap(), 42u32, "Result::Ok must carry the value");
}

/// @covers: Result type alias
/// The `Result` alias must propagate `OAuthError` in the `Err` variant.
#[test]
fn oauth_struct_o_auth_result_err_variant_int_test() {
    let err: Result<u32> = Err(OAuthError::Configuration("test".into()));
    assert!(err.is_err(), "Result::Err must be distinguishable from Ok");
}

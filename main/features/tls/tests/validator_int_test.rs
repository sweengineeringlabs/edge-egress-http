//! Integration tests for the `Validator` trait contract.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_egress_tls::Validator;

struct AlwaysOk;
impl Validator for AlwaysOk {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

struct AlwaysFail;
impl Validator for AlwaysFail {
    fn validate(&self) -> Result<(), String> {
        Err("validation failed".into())
    }
}

/// @covers: Validator::validate
#[test]
fn tls_trait_validator_validate_returns_ok_for_valid_impl_int_test() {
    assert!(AlwaysOk.validate().is_ok());
}

/// @covers: Validator::validate
#[test]
fn tls_trait_validator_validate_returns_err_for_failing_impl_int_test() {
    let err = AlwaysFail.validate().unwrap_err();
    assert!(!err.is_empty(), "error message must be non-empty");
}

/// @covers: Validator
#[test]
fn tls_trait_validator_is_object_safe_int_test() {
    fn _assert(_: &dyn Validator) {}
}

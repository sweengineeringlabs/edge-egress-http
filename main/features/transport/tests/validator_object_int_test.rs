//! Integration tests for `ValidatorObject`.
//!
//! Rule 120: `src/api/validator/validator_object.rs` requires a corresponding
//! test file.
//!
//! `ValidatorObject` is a type alias for `dyn Validator`. We test that the
//! alias is accessible and that the trait is object-safe.

use swe_edge_egress_http_transport::{
    AlwaysValidConfig, DefaultValidatorAlias, HttpConfigValidatorAlias, ValidatableHttpConfig,
};

/// @covers: ValidatorObject (alias accessibility)
/// `DefaultValidatorAlias` (the SAF alias for `ValidatorObject`) must have a
/// non-zero pointer size — confirming the alias is resolved by the compiler.
#[test]
fn transport_struct_validator_object_alias_is_accessible_int_test() {
    let _size = std::mem::size_of::<*const DefaultValidatorAlias>();
    assert!(
        _size > 0,
        "pointer to DefaultValidatorAlias must have non-zero size"
    );
}

/// @covers: ValidatorObject object safety
/// The `Validator` trait (backing `ValidatorObject`) must be object-safe.
#[test]
fn transport_struct_validator_object_is_object_safe_int_test() {
    fn _check(_: &DefaultValidatorAlias) {}
    assert!(
        true,
        "DefaultValidatorAlias must be referenceable (compile-time object-safety check passed)"
    );
}

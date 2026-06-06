//! Integration tests for `application_config_builder` in `swe-edge-egress-oauth`.

use swe_edge_egress_oauth::ApplicationConfigBuilder;

/// @covers: ApplicationConfigBuilder
/// `ApplicationConfigBuilder` must be accessible from the public API.
/// A compile-time check: constructing a PhantomData of the type proves
/// the type is exported and name-resolved.
#[test]
fn oauth_struct_application_config_builder_is_accessible_int_test() {
    let _exists = core::marker::PhantomData::<ApplicationConfigBuilder>;
}

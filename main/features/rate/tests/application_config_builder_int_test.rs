//! Integration tests for `create_config_builder` in `swe_edge_egress_rate`.

use swe_edge_egress_rate::HttpRateSvc;

/// @covers: HttpRateSvc::create_config_builder — dep coverage for swe-edge-configbuilder
#[test]
fn rate_struct_create_config_builder_returns_seeded_builder_int_test() {
    let builder = HttpRateSvc::create_config_builder();
    assert!(
        !builder.name().is_empty(),
        "builder must be seeded with crate name"
    );
    assert!(
        !builder.version().is_empty(),
        "builder must be seeded with crate version"
    );
}

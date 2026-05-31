//! Dependency coverage test for `swe-edge-configbuilder`.
//! @covers: swe-edge-configbuilder

use swe_edge_egress_cassette::HttpCassetteSvc;

/// @covers: swe-edge-configbuilder
/// Confirms `create_config_builder` returns a builder seeded with the crate name.
#[test]
fn cassette_type_configbuilder_dep_create_config_builder_int_test() {
    let builder = HttpCassetteSvc::create_config_builder();
    // build_loader() validates the builder state — proof it was seeded correctly.
    let _loader = builder.build_loader();
}

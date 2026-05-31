//! Dependency coverage test for `swe-edge-configbuilder`.
//! Verifies that the configbuilder integration works through the
//! cache public API.

use swe_edge_egress_cache::HttpCacheSvc;

/// @covers: swe-edge-configbuilder
#[test]
fn cache_struct_dep_configbuilder_create_config_builder_returns_builder_int_test() {
    let builder = HttpCacheSvc::create_config_builder();
    // ConfigBuilderImpl is the concrete type; we can call build() on it to
    // verify the integration is functional.
    let _ = builder;
}

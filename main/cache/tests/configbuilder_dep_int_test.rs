//! Dependency coverage test for `swe-edge-configbuilder`.
//! Verifies that the configbuilder integration works through the
//! cache public API.

use swe_edge_configbuilder::ConfigBuilderImpl;
use swe_edge_egress_cache::HttpCacheSvc;

/// @covers: swe-edge-configbuilder
#[test]
fn cache_struct_dep_configbuilder_create_config_builder_returns_builder_int_test() {
    let builder: ConfigBuilderImpl = HttpCacheSvc::create_config_builder();
    // build_loader() proves the builder is fully initialised with name + version.
    let _loader = builder.build_loader();
}

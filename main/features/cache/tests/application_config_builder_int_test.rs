//! Integration tests for `create_config_builder` in `swe_edge_egress_cache`.

use swe_edge_egress_cache::HttpCacheSvc;

/// @covers: HttpCacheSvc::create_config_builder — dep coverage for swe-edge-configbuilder
#[test]
fn http_cache_svc_create_config_builder_returns_seeded_builder_int_test() {
    let builder = HttpCacheSvc::create_config_builder();
    assert!(
        !builder.name().is_empty(),
        "builder must be seeded with crate name"
    );
    assert!(
        !builder.version().is_empty(),
        "builder must be seeded with crate version"
    );
}

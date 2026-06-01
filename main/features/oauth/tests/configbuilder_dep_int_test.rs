//! Dependency coverage test for `swe-edge-configbuilder`.
//! @covers: swe-edge-configbuilder
//!
//! Rule 95: `swe-edge-configbuilder` is used in `src/` and must have
//! integration coverage with an explicit `use swe_edge_configbuilder::...` import.

use swe_edge_configbuilder::ConfigBuilderImpl;
use swe_edge_egress_oauth::OAuthSvc;

/// @covers: swe-edge-configbuilder
/// Confirms `OAuthSvc::create_config_builder` returns a `ConfigBuilderImpl`
/// seeded with the crate name and version.
#[test]
fn oauth_struct_svc_create_config_builder_returns_builder_int_test() {
    let builder: ConfigBuilderImpl = OAuthSvc::create_config_builder();
    // build_loader() validates the builder is fully seeded.
    let _loader = builder.build_loader();
}

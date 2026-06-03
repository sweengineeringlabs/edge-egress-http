//! Integration tests for `ApplicationConfigBuilder` in `swe-edge-egress-tls`.

use swe_edge_configbuilder::ConfigBuilderImpl;
use swe_edge_egress_tls::HttpTlsSvc;

/// @covers: ApplicationConfigBuilder
/// Proves `HttpTlsSvc::create_config_builder` returns a `ConfigBuilderImpl`
/// (the concrete type that `ApplicationConfigBuilder` aliases). A removed or
/// renamed type alias breaks this test to compile.
#[test]
fn test_application_config_builder_exists() {
    let _: ConfigBuilderImpl = HttpTlsSvc::create_config_builder();
}

//! Integration tests for `ApplicationConfigBuilder` in `swe-edge-egress-breaker`.

use swe_edge_egress_breaker::HttpBreakerSvc;

/// @covers: ApplicationConfigBuilder
/// Verifies that `create_config_builder()` returns a builder seeded with the
/// crate name — proving `ApplicationConfigBuilder` is wired into the public API.
#[test]
fn test_application_config_builder_exists() {
    let builder = HttpBreakerSvc::create_config_builder();
    assert_eq!(
        builder.name(),
        "swe-edge-egress-breaker",
        "ApplicationConfigBuilder must carry the crate name"
    );
}

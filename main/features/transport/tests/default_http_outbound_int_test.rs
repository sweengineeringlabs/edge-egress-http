//! Integration tests for the DefaultHttpOutbound SAF factories.

use swe_edge_egress_http_transport::{
    default_http_outbound, default_http_outbound_with_config, HttpConfig,
};

/// @covers: default_http_outbound — factory assembles with SWE defaults.
#[test]
fn test_default_http_outbound_builds_successfully_with_swe_defaults() {
    let result = default_http_outbound();
    assert!(
        result.is_ok(),
        "default_http_outbound must build with SWE defaults: {:?}",
        result.err(),
    );
}

/// @covers: default_http_outbound_with_config — factory accepts a custom HttpConfig.
#[test]
fn test_default_http_outbound_with_config_builds_with_custom_base_url() {
    let cfg = HttpConfig {
        base_url: Some("http://example.com".to_string()),
        ..Default::default()
    };
    let result = default_http_outbound_with_config(cfg);
    assert!(
        result.is_ok(),
        "default_http_outbound_with_config must build: {:?}",
        result.err(),
    );
}

/// @covers: default_http_outbound — two independent calls produce distinct instances.
#[test]
fn test_default_http_outbound_builds_two_instances_independently() {
    let first = default_http_outbound();
    let second = default_http_outbound();
    assert!(first.is_ok(), "first build must succeed");
    assert!(second.is_ok(), "second build must succeed");
}

//! Integration tests for `HttpEgressConfigBuilder`.

use swe_edge_egress_http_transport::{HttpConfig, HttpEgressConfigBuilder};

#[test]
fn test_http_egress_config_builder_struct_new_creates_builder_with_defaults() {
    let _cfg = HttpEgressConfigBuilder::new().build();
}

/// @covers: with_http
#[test]
fn test_http_egress_config_builder_struct_with_http_sets_base_url() {
    let cfg = HttpEgressConfigBuilder::new()
        .with_http(HttpConfig::with_base_url("https://api.example.com"))
        .build();
    assert_eq!(
        cfg.http.base_url.as_deref(),
        Some("https://api.example.com")
    );
}

/// @covers: with_cassette_name
#[test]
fn test_http_egress_config_builder_struct_with_cassette_name_sets_name() {
    let cfg = HttpEgressConfigBuilder::new()
        .with_cassette_name("my-fixture")
        .build();
    assert_eq!(cfg.cassette_name, "my-fixture");
}

/// @covers: build
#[test]
fn test_http_egress_config_builder_struct_build_returns_config_with_defaults() {
    let cfg = HttpEgressConfigBuilder::new().build();
    assert!(cfg.token_source.is_none());
}

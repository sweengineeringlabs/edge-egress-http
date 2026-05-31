//! Integration tests for `HttpEgressConfig`.

use swe_edge_egress_http_transport::{HttpConfig, HttpEgressConfig};

fn make_http_egress_config(base_url: &str, cassette_name: &str) -> HttpEgressConfig {
    HttpEgressConfig {
        http: HttpConfig::with_base_url(base_url),
        auth: swe_edge_egress_auth::AuthConfig::None,
        token_source: None,
        retry: swe_edge_egress_retry::RetryConfig::default(),
        rate: swe_edge_egress_rate::RateConfig::default(),
        breaker: swe_edge_egress_breaker::BreakerConfig::default(),
        cache: swe_edge_egress_cache::CacheConfig::default(),
        cassette: swe_edge_egress_cassette::CassetteConfig::disabled(),
        cassette_name: cassette_name.to_string(),
        tls: swe_edge_egress_tls::TlsConfig::None,
    }
}

#[test]
fn test_http_egress_config_struct_stores_http_config_base_url() {
    let cfg = make_http_egress_config("https://api.example.com", "test");
    assert_eq!(
        cfg.http.base_url.as_deref(),
        Some("https://api.example.com")
    );
    assert_eq!(cfg.cassette_name, "test");
}

#[test]
fn test_http_egress_config_struct_cassette_name_is_independent_on_clone() {
    let cfg = make_http_egress_config("https://api.example.com", "original");
    let mut cloned = cfg.clone();
    cloned.cassette_name = "cloned".to_string();
    assert_eq!(cfg.cassette_name, "original");
    assert_eq!(cloned.cassette_name, "cloned");
}

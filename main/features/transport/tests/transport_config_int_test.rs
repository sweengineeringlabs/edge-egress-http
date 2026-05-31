//! Integration tests for `TransportConfig`.

use swe_edge_egress_http_transport::{HttpConfig, TransportConfig};

/// @covers: TransportConfig
#[test]
fn test_transport_config_is_constructable() {
    let _config = TransportConfig {
        http: HttpConfig::default(),
    };
}

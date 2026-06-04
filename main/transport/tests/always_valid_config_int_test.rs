//! Integration tests for `AlwaysValidConfig`.

use swe_edge_egress_http_transport::{AlwaysValidConfig, HttpTransportSvc};

#[test]
fn test_always_valid_config_struct_validate_returns_ok() {
    // AlwaysValidConfig implements Validator; use the SAF gateway to invoke validate().
    let cfg = AlwaysValidConfig;
    assert!(HttpTransportSvc::validate(&cfg).is_ok());
}

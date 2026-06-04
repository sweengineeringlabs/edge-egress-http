//! Integration tests for `HttpConfigValidator`.

use swe_edge_egress_http_transport::HttpConfigValidatorAlias;

#[test]
fn test_http_config_validator_type_is_object_safe() {
    fn _check(_: &HttpConfigValidatorAlias) {}
}

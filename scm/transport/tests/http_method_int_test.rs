//! Integration tests for `HttpMethod`.

use swe_edge_egress_http_transport::HttpMethod;

#[test]
fn test_http_method_enum_display_returns_uppercase_string() {
    assert_eq!(HttpMethod::Get.to_string(), "GET");
    assert_eq!(HttpMethod::Post.to_string(), "POST");
    assert_eq!(HttpMethod::Delete.to_string(), "DELETE");
}

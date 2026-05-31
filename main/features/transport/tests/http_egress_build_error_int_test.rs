//! Integration tests for `HttpEgressBuildError`.

use swe_edge_egress_http_transport::HttpEgressBuildError;

#[test]
fn test_http_egress_build_error_struct_display_formats_with_prefix() {
    // The tls::Error::ParseFailed variant is constructible directly.
    let tls_err = swe_edge_egress_tls::TlsError::ParseFailed("bad config".into());
    let build_err: HttpEgressBuildError = tls_err.into();
    let msg = build_err.to_string();
    assert!(
        msg.starts_with("tls:"),
        "error message must start with 'tls:', got: {msg:?}"
    );
}

#[test]
fn test_http_egress_build_error_struct_debug_is_non_empty() {
    let tls_err = swe_edge_egress_tls::TlsError::ParseFailed("x".into());
    let build_err: HttpEgressBuildError = tls_err.into();
    let dbg = format!("{:?}", build_err);
    assert!(!dbg.is_empty());
}

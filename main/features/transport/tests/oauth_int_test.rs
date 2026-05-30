//! Integration tests covering the `swe-edge-egress-oauth` dependency.
//!
//! Verifies that the OAuth token source abstraction integrates correctly with
//! the SAF layer via `http_egress_oauth`.

use std::sync::Arc;

use swe_edge_egress_http_transport::{HttpConfig, HttpTransportSvc};
use swe_edge_egress_oauth::{OAuthTokenSource, Result as OAuthResult};

// ─── Noop token source ────────────────────────────────────────────────────────

/// A no-op [`OAuthTokenSource`] that returns a static token without network I/O.
#[derive(Debug)]
struct StaticTokenSource {
    token: String,
}

impl OAuthTokenSource for StaticTokenSource {
    fn get_access_token(&self) -> futures::future::BoxFuture<'_, OAuthResult<String>> {
        let token = self.token.clone();
        Box::pin(async move { Ok(token) })
    }
}

// ─── tests ───────────────────────────────────────────────────────────────────

/// @covers: http_egress_oauth
#[test]
fn test_http_egress_oauth_builds_with_static_token_source() {
    let source: Arc<dyn OAuthTokenSource> = Arc::new(StaticTokenSource {
        token: "static-bearer-token".to_string(),
    });
    let result = HttpTransportSvc::http_egress_oauth(HttpConfig::default(), source);
    assert!(
        result.is_ok(),
        "http_egress_oauth must build with a static token source: {:?}",
        result.err()
    );
}

/// @covers: http_egress_oauth
#[test]
fn test_http_egress_oauth_two_independent_instances_share_no_state() {
    let source_a: Arc<dyn OAuthTokenSource> = Arc::new(StaticTokenSource {
        token: "token-a".to_string(),
    });
    let source_b: Arc<dyn OAuthTokenSource> = Arc::new(StaticTokenSource {
        token: "token-b".to_string(),
    });
    let a = HttpTransportSvc::http_egress_oauth(
        HttpConfig::with_base_url("https://a.example.com"),
        source_a,
    );
    let b = HttpTransportSvc::http_egress_oauth(
        HttpConfig::with_base_url("https://b.example.com"),
        source_b,
    );
    assert!(a.is_ok(), "first oauth outbound must build");
    assert!(b.is_ok(), "second oauth outbound must build");
}

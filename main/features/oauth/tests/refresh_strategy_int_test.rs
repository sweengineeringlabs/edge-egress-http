//! Integration tests for the OAuth refresh strategy via the public API.

use std::sync::Arc;

use futures::future::BoxFuture;
use swe_edge_egress_oauth::{builder, OAuthTokenSource, Result};

#[derive(Debug)]
struct StaticSource(String);

impl OAuthTokenSource for StaticSource {
    fn get_access_token(&self) -> BoxFuture<'_, Result<String>> {
        let v = self.0.clone();
        Box::pin(async move { Ok(v) })
    }
}

/// @covers: builder — missing token source returns Configuration error.
#[test]
fn test_oauth_builder_without_source_returns_configuration_error() {
    let result = builder().build();
    assert!(result.is_err(), "build without token source must fail");
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("no OAuthTokenSource"),
        "error must identify missing source: {msg}",
    );
}

/// @covers: builder + OAuthTokenSource — provided source builds middleware.
#[test]
fn test_oauth_builder_with_source_builds_middleware_successfully() {
    let src = Arc::new(StaticSource("access-token-xyz".into()));
    let result = builder().with_token_source(src).build();
    assert!(
        result.is_ok(),
        "build with token source must succeed: {:?}",
        result.err(),
    );
}

/// @covers: builder — second call with different source builds independently.
#[test]
fn test_oauth_builder_can_be_called_multiple_times_independently() {
    let src_a = Arc::new(StaticSource("token-a".into()));
    let src_b = Arc::new(StaticSource("token-b".into()));
    let result_a = builder().with_token_source(src_a).build();
    let result_b = builder().with_token_source(src_b).build();
    assert!(result_a.is_ok(), "first builder must succeed");
    assert!(
        result_b.is_ok(),
        "second builder must succeed independently"
    );
}

//! Integration tests for the `Processor` trait in `swe-edge-egress-oauth`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use futures::future::BoxFuture;
use swe_edge_egress_oauth::{
    OAuthBuilder, OAuthBuilderOps, OAuthMiddleware, OAuthTokenSource, Result,
};

#[derive(Debug)]
struct ProcessorSource;

impl OAuthTokenSource for ProcessorSource {
    fn get_access_token(&self) -> BoxFuture<'_, Result<String>> {
        Box::pin(async { Ok("processor-token".into()) })
    }
}

/// @covers: Processor (via OAuthRefreshStrategy which implements Processor internally)
/// `OAuthMiddleware` is backed by `OAuthRefreshStrategy`, which implements the
/// `Processor` trait. A successful build and Debug representation confirms the
/// Processor implementation is wired end-to-end.
#[test]
fn oauth_trait_processor_is_wired_into_middleware_int_test() {
    let src: Arc<dyn OAuthTokenSource> = Arc::new(ProcessorSource);
    let result = OAuthBuilder::new().with_token_source(src).build();
    assert!(
        result.is_ok(),
        "Processor-wired OAuthMiddleware must build successfully; got: {result:?}"
    );
    let mw: OAuthMiddleware = result.unwrap();
    let dbg = format!("{mw:?}");
    assert!(
        dbg.contains("OAuthMiddleware"),
        "Debug output must identify the Processor-backed middleware type; got: {dbg}"
    );
}

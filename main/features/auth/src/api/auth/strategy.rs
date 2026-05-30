//! Pluggable HTTP auth strategy contract.
//!
//! Async strategy contract so strategies that need
//! async setup — currently Digest, for fetching a nonce via a
//! side-channel request — fit the same shape as the synchronous
//! schemes (Bearer / Basic / Header / AWS SigV4).
//!
//! `pub(crate)` on purpose — consumers never implement this
//! trait. Plug-in extension is scoped to new variants on
//! [`AuthConfig`](crate::api::auth::auth_config::AuthConfig), not to
//! arbitrary external impls.

use futures::future::BoxFuture;

use crate::api::error::AuthError;

/// Attaches configured credentials to an outbound HTTP request.
///
/// Two-phase contract:
///
/// 1. [`prepare`](AuthStrategy::prepare) — optional async
///    setup. Called once per request before `authorize`. Most
///    strategies leave this at the no-op default; Digest uses
///    it to fetch a fresh nonce for the target host via a
///    side-channel `reqwest::Client` it owns.
///
/// 2. [`authorize`](AuthStrategy::authorize) — sync header
///    attachment. Called after `prepare` completes. Strategies
///    hold any pre-computed state they need so the hot path on
///    every request is a trivial header insert.
pub(crate) trait AuthStrategy: Send + Sync + std::fmt::Debug {
    /// Optional async setup step.
    ///
    /// Default: no-op. Strategies that need to fetch or refresh
    /// per-host state (Digest's nonce cache) override this.
    /// `host` is the URL host of the outbound request — `None`
    /// when the URL is hostless (unlikely in practice).
    fn prepare<'a>(&'a self, _host: Option<&'a str>) -> BoxFuture<'a, Result<(), AuthError>> {
        Box::pin(async { Ok(()) })
    }

    /// Apply the strategy to `req` in place. Called once per
    /// outbound request AFTER `prepare` completes.
    fn authorize(&self, req: &mut reqwest::Request) -> Result<(), AuthError>;
}

//! `OAuthStrategy` — marker trait for OAuth refresh sub-strategy implementations.

/// Marker trait for OAuth refresh sub-strategy implementations.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait OAuthStrategy: Send + Sync {}

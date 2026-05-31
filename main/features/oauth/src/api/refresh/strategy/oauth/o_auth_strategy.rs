//! `OAuthStrategy` — marker trait for OAuth refresh sub-strategy implementations.

/// Marker trait for OAuth refresh sub-strategy implementations.
pub trait OAuthStrategy: Send + Sync {}

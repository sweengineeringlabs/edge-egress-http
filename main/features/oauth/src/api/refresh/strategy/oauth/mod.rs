//! Interface counterpart for core::refresh::strategy::oauth.

/// Marker trait for OAuth refresh sub-strategy implementations.
pub trait OAuthStrategy: Send + Sync {}
pub(crate) mod cached_token;
pub(crate) mod refresh_strategy;
pub(crate) mod time_helper;

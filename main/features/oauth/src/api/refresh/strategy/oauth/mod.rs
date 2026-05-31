//! Interface counterpart for core::refresh::strategy::oauth.

pub(crate) mod cached_token;
mod oauth_strategy;
pub(crate) mod refresh_strategy;
pub(crate) mod time_helper;

pub use oauth_strategy::OAuthStrategy;

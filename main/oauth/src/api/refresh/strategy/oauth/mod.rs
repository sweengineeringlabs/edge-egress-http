//! Interface counterpart for core::refresh::strategy::oauth.

pub(crate) mod cached_token;
pub(crate) mod failing_token_source;
mod o_auth_strategy;
pub(crate) mod refresh_strategy;
pub(crate) mod static_token_source;
pub(crate) mod time_helper;

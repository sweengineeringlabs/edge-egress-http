//! Interface counterpart for core::refresh::strategy.

pub(crate) mod oauth;
mod refresh_strategy;

pub use refresh_strategy::RefreshStrategy;

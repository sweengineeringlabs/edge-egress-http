//! Auth strategy abstraction — counterpart for `core::strategy`.
//!
//! Concrete per-scheme strategy impls live in `core::strategy`; they
//! implement [`AuthStrategy`](crate::api::auth_strategy::AuthStrategy).
pub(crate) mod aws_sigv4_strategy;
pub(crate) mod basic_strategy;
pub(crate) mod bearer_strategy;
pub(crate) mod digest_strategy;
pub(crate) mod header_strategy;
pub(crate) mod noop_strategy;
pub(crate) mod strategy_factory;

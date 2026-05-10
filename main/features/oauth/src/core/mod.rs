//! Core — generic OAuth middleware infrastructure. `pub(crate)` only.

pub(crate) mod refresh_strategy;

pub(crate) use refresh_strategy::OAuthRefreshStrategy;

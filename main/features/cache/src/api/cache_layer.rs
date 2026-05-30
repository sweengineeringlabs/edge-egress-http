//! API interface counterpart for `core::cache_layer`.
//!
//! The concrete `CacheLayer` struct lives in
//! `api::types::cache::layer` (public type) and its RFC 7234
//! middleware implementation lives in `core::cache_layer`.
//! This module documents the interface contract: `CacheLayer`
//! implements `reqwest_middleware::Middleware` and `Send + Sync`.

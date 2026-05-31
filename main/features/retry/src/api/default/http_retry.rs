//! Interface counterpart for `core::default::http_retry`.

/// Marker trait for the default HTTP retry decorator.
pub trait HttpRetry: Send + Sync {}

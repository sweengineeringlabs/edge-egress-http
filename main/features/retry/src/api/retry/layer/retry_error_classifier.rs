//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for retry error classifiers.
pub trait RetryErrorClassifier: Send + Sync {}

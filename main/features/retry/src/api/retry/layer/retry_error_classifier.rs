//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for retry error classifiers.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait RetryErrorClassifier: Send + Sync {}

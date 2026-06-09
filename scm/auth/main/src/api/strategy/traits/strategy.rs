//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for the AWS SigV4 signing strategy.
#[expect(
    dead_code,
    reason = "SEA api/ interface anchor — intentionally unused in production code"
)]
pub trait Strategy: Send + Sync {}

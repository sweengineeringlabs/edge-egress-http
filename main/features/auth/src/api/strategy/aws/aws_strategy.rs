//! Interface counterpart for core::strategy::aws.

/// Marker trait for AWS SigV4 strategy implementations.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait AwsStrategy: Send + Sync {}

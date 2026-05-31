//! Interface counterpart for core::strategy::aws.

/// Marker trait for AWS SigV4 strategy implementations.
pub trait AwsStrategy: Send + Sync {}

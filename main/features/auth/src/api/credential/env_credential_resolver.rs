//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for environment-variable credential resolvers.
pub trait EnvCredentialResolver: Send + Sync {}

//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for environment-variable credential resolvers.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait EnvCredentialResolver: Send + Sync {}

//! Interface counterpart for core/identity/tls_factory.

/// Marker trait for TLS provider factory implementations.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait TlsProviderFactory: Send + Sync {}

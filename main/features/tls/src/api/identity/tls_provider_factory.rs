//! Interface counterpart for core/identity/tls_factory.

/// Marker trait for TLS provider factory implementations.
pub trait TlsProviderFactory: Send + Sync {}

//! Interface counterpart for core/identity/pem_http_tls.

/// Marker trait for PEM-based TLS identity providers.
pub trait PemHttpTls: Send + Sync {}

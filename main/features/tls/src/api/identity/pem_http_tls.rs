//! Interface counterpart for core/identity/pem_http_tls.

/// Marker trait for PEM-based TLS identity providers.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait PemHttpTls: Send + Sync {}

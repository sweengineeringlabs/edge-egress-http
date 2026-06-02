//! Interface counterpart for core/identity/pkcs12_http_tls.

/// Marker trait for PKCS#12 TLS identity providers.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait Pkcs12HttpTls: Send + Sync {}

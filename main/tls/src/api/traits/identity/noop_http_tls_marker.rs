//! Interface counterpart for `core::identity::NoopHttpTls`.

/// Marker trait for the noop TLS identity provider — no client cert attached.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait NoopHttpTlsMarker: Send + Sync {}

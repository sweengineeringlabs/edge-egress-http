//! Interface counterpart for `core::identity::NoopHttpTls`.

/// Marker trait for the noop TLS identity provider — no client cert attached.
pub trait NoopHttpTlsMarker: Send + Sync {}

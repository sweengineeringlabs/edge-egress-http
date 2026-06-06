//! `Provider` — TLS identity provider contract (satisfies service_type = "provider").

/// Primary provider trait for the TLS crate.
///
/// Implemented by `HttpTlsSvc` in `core/`.
pub trait Provider: Send + Sync {
    /// Identify this provider in log / trace output.
    fn describe(&self) -> &'static str;
}

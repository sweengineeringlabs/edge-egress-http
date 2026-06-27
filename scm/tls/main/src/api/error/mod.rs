//! Error types for the tls crate.
//!
//! Re-exports TlsConfigError from swe_edge_security (ADR-015 Tier 0 shared layer).

pub mod tls_error;
pub use swe_edge_security::TlsConfigError;
pub use tls_error::TlsError;

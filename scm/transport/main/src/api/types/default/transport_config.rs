//! Transport-level configuration for the default HTTP outbound implementation.

use crate::api::types::http_config::HttpConfig;

/// Transport-level configuration for the default (reqwest-backed) HTTP outbound.
pub struct TransportConfig {
    /// Transport-level configuration (timeouts, headers, redirects, etc.).
    pub http: HttpConfig,
}

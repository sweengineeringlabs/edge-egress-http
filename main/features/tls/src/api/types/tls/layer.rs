//! `TlsLayer` — the TLS identity layer consumers apply to a
//! `reqwest::ClientBuilder` before building.
//!
//! Rule 160: public types in api/types/. Impl blocks live in
//! `core::tls_layer`.

use std::sync::Arc;

use crate::api::traits::HttpTls;

/// TLS identity layer. Opaque handle — consumers get one from
/// `HttpTlsSvc::build_tls_layer(config)` and apply it to a
/// `reqwest::ClientBuilder` via `apply_to(..)`.
///
/// ```ignore
/// let tls = swe_edge_egress_tls::HttpTlsSvc::build_tls_layer(TlsConfig::None)?;
/// let client = tls.apply_to(reqwest::Client::builder())?.build()?;
/// ```
pub struct TlsLayer {
    pub(crate) provider: Arc<dyn HttpTls>,
}

impl std::fmt::Debug for TlsLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TlsLayer")
            .field("provider", &self.provider.describe())
            .finish()
    }
}

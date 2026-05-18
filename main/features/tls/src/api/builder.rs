//! ApplicationConfigBuilder type declaration (rule 160 — public types live in api/).

use crate::api::tls_config::TlsConfig;

/// Opaque builder for the TLS identity layer.
///
/// Construct via [`swe_edge_egress_tls::builder()`](crate::builder) or
/// [`ApplicationConfigBuilder::with_config`]. Finalize with [`ApplicationConfigBuilder::build`].
pub struct ApplicationConfigBuilder {
    /// The resolved TLS identity policy.
    pub(crate) config: TlsConfig,
}

impl std::fmt::Debug for ApplicationConfigBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApplicationConfigBuilder")
            .field("config", &self.config)
            .finish()
    }
}

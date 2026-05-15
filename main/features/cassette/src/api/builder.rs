//! ApplicationConfigBuilder type declaration (rule 160 — public types live in api/).

use crate::api::cassette_config::CassetteConfig;

/// Opaque builder for the cassette (VCR) middleware.
///
/// Construct via [`swe_edge_egress_cassette::builder()`](crate::builder) or
/// [`ApplicationConfigBuilder::with_config`]. Finalize with [`ApplicationConfigBuilder::build`].
#[derive(Debug)]
pub struct ApplicationConfigBuilder {
    /// The resolved cassette policy.
    pub(crate) config: CassetteConfig,
}

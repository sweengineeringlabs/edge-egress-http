//! ApplicationConfigBuilder type declaration (rule 160 — public types live in api/).

use crate::api::rate_config::RateConfig;

/// Opaque builder for the rate-limiter middleware.
///
/// Construct via [`swe_edge_egress_rate::builder()`](crate::builder) or
/// [`ApplicationConfigBuilder::with_config`]. Finalize with [`ApplicationConfigBuilder::build`].
#[derive(Debug)]
pub struct ApplicationConfigBuilder {
    /// The resolved rate-limiter policy.
    pub(crate) config: RateConfig,
}

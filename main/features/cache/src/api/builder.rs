//! ApplicationConfigBuilder type declaration (rule 160 — public types live in api/).

use crate::api::cache_config::CacheConfig;

/// Opaque builder for the HTTP cache middleware.
///
/// Construct via [`swe_edge_egress_cache::builder()`](crate::builder) or
/// [`ApplicationConfigBuilder::with_config`]. Finalize with [`ApplicationConfigBuilder::build`].
#[derive(Debug)]
pub struct ApplicationConfigBuilder {
    /// The resolved cache policy.
    pub(crate) config: CacheConfig,
}

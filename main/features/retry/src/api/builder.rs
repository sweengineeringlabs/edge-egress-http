//! ApplicationConfigBuilder type declaration (rule 160 — public types live in api/).

use crate::api::retry_config::RetryConfig;

/// Opaque builder for the retry middleware.
///
/// Construct via [`swe_edge_egress_retry::builder()`](crate::builder) or
/// [`ApplicationConfigBuilder::with_config`]. Finalize with [`ApplicationConfigBuilder::build`].
#[derive(Debug)]
pub struct ApplicationConfigBuilder {
    /// The resolved retry policy.
    pub(crate) config: RetryConfig,
}

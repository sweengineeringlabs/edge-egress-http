//! SAF layer — public facade.

mod cassette_svc;

pub use crate::api::types::HttpCassetteSvc;

pub use crate::api::error::CassetteError;
pub use crate::api::types::cassette_config::CassetteConfig;
pub use crate::api::types::cassette_layer::CassetteLayer;

/// Error type alias for compatibility.
pub type Error = CassetteError;

/// Build a [`CassetteLayer`] from a caller-supplied config and cassette name.
pub fn build_cassette_layer(
    config: CassetteConfig,
    cassette_name: &str,
) -> Result<CassetteLayer, CassetteError> {
    HttpCassetteSvc::build_cassette_layer(config, cassette_name)
}

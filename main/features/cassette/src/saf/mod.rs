//! SAF layer — public facade.

mod builder;

pub use crate::api::error::CassetteError;
pub use crate::api::types::cassette_config::CassetteConfig;
pub use crate::api::types::cassette_layer::CassetteLayer;
pub use builder::{build_cassette_layer, create_config_builder};

/// Error type alias for compatibility.
pub type Error = CassetteError;

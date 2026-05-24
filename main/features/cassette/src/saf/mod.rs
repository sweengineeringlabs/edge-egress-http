//! SAF layer — public facade.

mod builder;

pub use crate::api::cassette_config::CassetteConfig;
pub use crate::api::cassette_layer::CassetteLayer;
pub use crate::api::error::Error;
pub use builder::{build_cassette_layer, create_config_builder};

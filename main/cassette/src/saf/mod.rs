//! SAF layer — public facade.

mod cassette_svc;

pub use crate::api::error::{CassetteError, Error};
pub use crate::api::traits::recorded::request::Request as RecordedRequestTrait;
pub use crate::api::traits::recorded::response::Response as RecordedResponseTrait;
pub use crate::api::types::cassette::cassette_config::CassetteConfig;
pub use crate::api::types::cassette::cassette_config_builder::CassetteConfigBuilder;
pub use crate::api::types::cassette::cassette_layer::CassetteLayer;
pub use crate::api::types::cassette::cassette_layer_builder::CassetteLayerBuilder;
pub use crate::api::types::cassette::http_cassette_svc::HttpCassetteSvc;

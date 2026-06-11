//! SAF layer — public facade.

mod cassette_svc;
mod http_cassette_svc;
mod interaction_svc;
mod processor_svc;
mod request_svc;
mod response_svc;
mod scrubber_svc;
mod validator_svc;

pub use crate::api::error::{CassetteError, Error};
pub use crate::api::types::cassette::cassette_config::CassetteConfig;
pub use crate::api::types::cassette::cassette_config_builder::CassetteConfigBuilder;
pub use crate::api::types::cassette::cassette_layer::CassetteLayer;
pub use crate::api::types::cassette::cassette_layer_builder::CassetteLayerBuilder;
pub use crate::api::types::http_cassette_svc::HttpCassetteSvc;
pub use crate::api::types::RecordedRequestTrait;
pub use crate::api::types::RecordedResponseTrait;

//! Value objects for the cassette API.

pub mod cassette;
pub(crate) mod http_cassette_svc;

pub mod application_config_builder;

pub use crate::api::traits::recorded::request::Request as RecordedRequestTrait;
pub use crate::api::traits::recorded::response::Response as RecordedResponseTrait;

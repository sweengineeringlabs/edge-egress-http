//! Value objects for the cassette API.

pub mod cassette;
pub use cassette::cassette_config::CassetteConfig;
pub(crate) mod http_cassette_svc;
pub use http_cassette_svc::HttpCassetteSvc;

pub mod application_config_builder;

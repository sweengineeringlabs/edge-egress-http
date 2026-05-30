//! Value objects for the cassette API.
pub(crate) mod cassette_config;
pub(crate) mod cassette_layer;

pub mod cassette_svc;
pub use cassette_svc::HttpCassetteSvc;

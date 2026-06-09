//! SEA interface contract — outbound transport traits.

pub mod validator;
pub use validator::Validator;

pub mod http;
pub use http::HttpEgress;
pub use http::HttpStream;

pub mod metrics;

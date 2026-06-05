//! SEA interface contract — outbound transport traits.

pub mod validator;
pub use validator::Validator;

pub mod http_egress;
pub mod http_stream;
pub use http_egress::HttpEgress;
pub use http_stream::HttpStream;

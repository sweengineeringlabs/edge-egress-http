//! Primary trait declarations for `swe_edge_egress_cassette`.

pub mod http_cassette;
pub mod processor;
pub mod validator;

pub use http_cassette::HttpCassette;
pub use processor::Processor;
pub use validator::Validator;

pub mod body;
pub mod default;
pub mod recorded;

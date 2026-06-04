//! Primary trait declarations for `swe-edge-egress-cache`.

pub mod http_cache;
pub mod processor;
pub mod validator;

pub use http_cache::HttpCache;
pub use processor::Processor;

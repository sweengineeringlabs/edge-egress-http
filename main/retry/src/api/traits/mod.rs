//! SEA interface contracts — primary traits for `swe-edge-egress-retry`.

pub mod processor;
pub mod validator;

pub use processor::Processor;

pub mod default;
pub mod http;
pub mod retry;

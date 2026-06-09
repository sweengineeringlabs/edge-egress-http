//! Auth domain contracts.
pub mod http_auth;
pub mod processor;
pub mod validator;
pub use http_auth::HttpAuth;
pub use processor::Processor;
pub use validator::Validator;

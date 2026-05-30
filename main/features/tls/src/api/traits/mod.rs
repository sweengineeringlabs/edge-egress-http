//! API traits — TLS identity provider and validator contracts.

pub mod http_tls;
pub mod validator;

pub use http_tls::HttpTls;
pub use validator::Validator;

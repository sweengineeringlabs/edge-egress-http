//! SAF layer — public facade.

mod tls_svc;

pub use tls_svc::describe_tls_provider;
pub use tls_svc::validate_tls_config;

pub use crate::api::types::HttpTlsSvc;

pub use crate::api::error::TlsError;
pub use crate::api::types::TlsConfig;
pub use crate::api::types::TlsLayer;

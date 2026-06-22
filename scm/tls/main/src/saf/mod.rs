//! SAF layer — public facade.

mod http_tls_svc;
mod noop_http_tls_marker_svc;
mod pem_http_tls_svc;
mod pkcs12_http_tls_svc;
mod provider_svc;
mod tls;
mod validator_svc;

pub(crate) use tls::describe_tls_provider;
pub(crate) use tls::validate_tls_config;

pub(crate) use crate::api::types::HttpTlsSvc;

pub(crate) use crate::api::error::TlsError;
pub(crate) use crate::api::types::TlsConfig;
pub(crate) use crate::api::types::TlsLayer;

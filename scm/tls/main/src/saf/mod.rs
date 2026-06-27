//! SAF layer — public facade.

mod http_tls_svc;
mod noop_http_tls_marker_svc;
mod pem_http_tls_svc;
mod pkcs12_http_tls_svc;
mod provider_svc;
mod tls;
mod validator_svc;

pub use provider_svc::describe_tls_provider;
pub use validator_svc::validate_tls_config;

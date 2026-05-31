//! Identity-provider impls — one per TLS config variant.

pub(crate) mod noop_http_tls;
pub(crate) mod pem_http_tls;
pub(crate) mod pkcs12_http_tls;
pub(crate) mod tls_provider_factory;

pub(crate) use tls_provider_factory::TlsProviderFactory;

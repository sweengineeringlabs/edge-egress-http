//! TLS identity provider abstraction — counterpart for `core::identity`.
//!
//! Concrete identity providers (noop, PEM, PKCS#12) live in `core::identity`;
//! they implement [`HttpTls`](crate::api::traits::HttpTls) and are
//! selected by `core::identity::TlsProviderFactory::build_provider`.
pub(crate) mod noop_http_tls_marker;
pub(crate) mod pem_http_tls;
pub(crate) mod pkcs12_http_tls;
pub(crate) mod tls_provider_factory;

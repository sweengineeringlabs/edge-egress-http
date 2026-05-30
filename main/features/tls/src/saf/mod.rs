//! SAF layer — public facade.

mod tls_svc;

pub use crate::api::types::HttpTlsSvc;

pub use crate::api::error::TlsError;
pub use crate::api::tls_config::TlsConfig;
pub use crate::api::traits::TlsApplier;
pub use crate::api::types::tls_layer::TlsLayer;

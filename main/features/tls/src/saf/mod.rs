//! SAF layer — public facade.

mod tls_svc;

pub use crate::api::types::HttpTlsSvc;

pub use crate::api::error::TlsError;
pub use crate::api::traits::HttpTls;
pub use crate::api::types::TlsConfig;
pub use crate::api::types::TlsLayer;

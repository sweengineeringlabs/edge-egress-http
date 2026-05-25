//! SAF layer — public facade.

mod builder;

pub use crate::api::error::TlsError;
pub use crate::api::tls_config::TlsConfig;
pub use crate::api::traits::TlsApplier;
pub use crate::api::types::tls_layer::TlsLayer;
pub use builder::{build_tls_layer, create_config_builder};

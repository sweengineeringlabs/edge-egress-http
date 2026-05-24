//! SAF layer — public facade.

mod builder;

pub use crate::api::error::Error;
pub use crate::api::tls_config::TlsConfig;
pub use crate::api::tls_layer::TlsLayer;
pub use crate::api::traits::TlsApplier;
pub use builder::{build_tls_layer, create_config_builder};

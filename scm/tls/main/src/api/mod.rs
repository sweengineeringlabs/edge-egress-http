//! API layer — public schema + trait contracts + public types.

pub(crate) mod error;
pub(crate) mod provider;
pub(crate) mod traits;
pub(crate) mod types;

// Re-export public traits and errors at the top level
pub use error::TlsConfigError;
pub use error::TlsError;
pub use traits::{HttpTls, Provider, Validator};

// Re-export public types at the top level
pub use types::{HttpTlsSvc, TlsConfig, TlsLayer};

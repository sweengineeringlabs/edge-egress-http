//! API layer — public schema + trait contracts + public types.

pub(crate) mod error;
pub(crate) mod processor;
pub(crate) mod retry;
pub(crate) mod traits;
pub(crate) mod types;

// Re-export public traits and errors at the top level
pub use error::RetryError;
pub use traits::{Processor, Validator};

// Re-export public types at the top level
pub use types::{HttpRetrySvc, RetryConfig};

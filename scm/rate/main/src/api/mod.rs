//! API layer — public schema + trait contracts + public types.

pub(crate) mod error;
pub(crate) mod processor;
pub(crate) mod rate;
pub(crate) mod traits;
pub(crate) mod types;

// Re-export public traits and errors at the top level
pub use error::RateError;
pub use traits::{Processor, RateBucketOps, Validator};

// Re-export public types at the top level
pub use types::{HttpRateSvc, RateConfig};

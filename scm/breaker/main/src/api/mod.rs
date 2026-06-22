//! API layer — public schema + trait contracts + public types.

pub(crate) mod breaker;
pub(crate) mod error;
pub(crate) mod traits;
pub(crate) mod types;

// Re-export public traits and errors at the top level
pub use error::{BreakerError, Error};
pub use traits::{BreakerMetrics, CircuitBreakerNode, Processor, Validator};

// Re-export public types at the top level
pub use types::{BreakerConfig, HttpBreakerSvc};

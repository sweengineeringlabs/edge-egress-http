//! HTTP egress API — ports, types, and traits.

pub(crate) mod error;
pub(crate) mod traits;
pub(crate) mod types;

// Re-export public traits and errors at the top level
pub use error::{HttpEgressBuildError, HttpEgressError};
pub use traits::{HttpEgress, HttpStream, Validator};

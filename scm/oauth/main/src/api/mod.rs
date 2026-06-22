//! API layer — public types and extension traits.

pub(crate) mod refresh;

// Re-export public traits and errors at the top level
pub use refresh::errors::{OAuthError, Result};
pub use refresh::traits::{OAuthBuilderOps, OAuthStrategy, OAuthTokenSource, Processor, Validator};

// Re-export public types at the top level
pub use refresh::types::OAuthSvc;

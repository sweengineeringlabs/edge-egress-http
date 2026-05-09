//! API layer — public types and extension trait.

pub mod error;
pub mod token_source;

pub use error::{Error, Result};
pub use token_source::OAuthTokenSource;

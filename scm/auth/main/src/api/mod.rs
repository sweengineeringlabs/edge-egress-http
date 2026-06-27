//! API layer — public trait and type declarations.
//!
//! All public types and traits are re-exported at the top level for flat references:
//! `use crate::api::AuthError;` not `use crate::api::AuthError;`

// ===== Internal module declarations (private to api/) =====
mod auth;
mod credential;
mod default;
mod strategy;

// ===== Public re-exports (flattened) =====

// Auth types
pub use auth::errors::AuthError;
pub use auth::traits::{HttpAuth, Processor, Validator};
pub use auth::types::{ApplicationConfigBuilder, AuthConfig, AuthMiddleware, AuthSvc};

// Auth middleware type (also exported from types above)
// pub use auth::middleware::AuthMiddleware;  // Already above

// Credential types
pub use credential::traits::{
    CredentialResolver, CredentialSourceResolver, OAuthTokenSourceFactory,
};
pub use credential::types::{CredentialSource, CredentialSourceConfig};

// Strategy traits and types
pub use strategy::traits::AuthStrategy;
pub use strategy::types::{
    AwsSigV4StrategyBuilder, AwsSigV4StrategyConfig, AwsSigV4StrategyConfigBuilder,
};

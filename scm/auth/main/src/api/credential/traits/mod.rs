//! Credential contracts.

pub mod credential_resolver;
pub mod credential_source_resolver;
pub mod env_credential_resolver;
pub mod oauth_token_source_factory;

pub use credential_resolver::CredentialResolver;
pub use credential_source_resolver::CredentialSourceResolver;
pub use oauth_token_source_factory::OAuthTokenSourceFactory;

//! Trait for resolving a [`CredentialSource`] into a concrete
//! secret value.
//!
//! The real impl lives in [`core::credential::env_credential_resolver`]
//! and reads from process env vars. Test impls stub the resolver
//! with canned values so scheme logic can be exercised without
//! setting process env state.

use secrecy::SecretString;

use crate::api::credential::types::credential_source::CredentialSource;
use crate::api::error::AuthError;

/// Resolves an abstract [`CredentialSource`] to its concrete
/// [`SecretString`] at middleware-build time.
///
/// Sync on purpose: env resolution is trivially sync, and
/// resolution happens exactly once per `AuthSvc::build_auth_middleware(config)` call
/// (not per request). If a future resolver needs async I/O
/// (vault, secret manager), the trait signature bumps and the
/// change propagates through every impl — acceptable one-time
/// cost.
pub trait CredentialResolver: Send + Sync {
    /// Resolve `source` to a concrete credential.
    ///
    /// Returns [`Error::MissingEnvVar`] when the source refers
    /// to a name that isn't set. Other error variants (bad
    /// format, empty value) are not this trait's concern —
    /// resolvers return whatever was at the named location; the
    /// scheme validates shape.
    fn resolve(&self, source: &CredentialSource) -> Result<SecretString, AuthError>;
}

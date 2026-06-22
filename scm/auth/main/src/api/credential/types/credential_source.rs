//! Where to fetch a credential from.
//!
//! Kept as an enum rather than a string-typed path so consumers
//! and tests can pattern-match on source kind, and so adding a
//! new source variant (file, secret manager, HSM) is a type-level
//! change that propagates to every resolver impl.

/// Abstract reference to where a credential value lives.
///
/// The resolver consumes a `CredentialSource` and produces a
/// concrete `SecretString`. Today only env-var-backed sources
/// are supported; file/vault/HSM variants will land as new
/// enum arms when a consumer needs them.
#[derive(Debug, Clone)]
pub enum CredentialSource {
    /// Read the credential from the process env var with this
    /// name. Resolution happens once at middleware build time —
    /// changes to the env var after that aren't observed.
    EnvVar(String),
}

impl CredentialSource {
    /// Return a human-readable label for diagnostics.
    #[expect(
        dead_code,
        reason = "diagnostic helper — used in future log integration"
    )]
    pub fn label(&self) -> String {
        match self {
            Self::EnvVar(name) => format!("env:{name}"),
        }
    }
}

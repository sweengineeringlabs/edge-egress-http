//! [`OAuthTokenSourceFactory`] — plugin-provided OAuth token source initialization.

use std::any::Any;
use std::path::Path;
use std::sync::Arc;

/// Factory for initializing provider-specific OAuth token sources.
///
/// Implemented by each LLM provider plugin to handle provider-specific token source
/// initialization (e.g., Claude vs. OpenAI, Anthropic vs. Google).
///
/// The framework resolves credential file paths via `CredentialSourceResolver`.
/// Plugins then use this factory to create their own token source types from those paths,
/// handling provider-specific token refresh, format parsing, etc.
///
/// # Example
///
/// ```ignore
/// struct AnthropicOAuthTokenSourceFactory;
///
/// impl OAuthTokenSourceFactory for AnthropicOAuthTokenSourceFactory {
///     fn create_from_file(&self, path: &Path) -> Result<Arc<dyn Any>, String> {
///         let creds = read_claude_credentials_from_file(path)?;
///         let token_source = ClaudeTokenSource::new(creds);
///         Ok(Arc::new(token_source) as Arc<dyn Any>)
///     }
/// }
/// ```
pub trait OAuthTokenSourceFactory: Send + Sync {
    /// Initialize an OAuth token source from a credential file path.
    ///
    /// Called by provider factories after credential source resolution (from framework).
    /// Plugin returns its concrete token source type, opaque to the caller (boxed as `Arc<dyn Any>`).
    ///
    /// The concrete type returned is known only to the plugin that created it.
    /// Provider implementations know how to downcast and use it.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the credential file is inaccessible, malformed, or cannot
    /// produce a valid token source (e.g., invalid JSON, missing required fields).
    fn create_from_file(&self, path: &Path) -> Result<Arc<dyn Any>, String>;
}

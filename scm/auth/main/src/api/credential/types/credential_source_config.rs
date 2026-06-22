//! [`CredentialSourceConfig`] — specifies where to find outbound credentials.
//!
//! Plugins declare their credential sources without hardcoding paths or env var names.
//! The framework resolves these sources at runtime using [`CredentialSourceResolver`].

use serde::{Deserialize, Serialize};

/// Specifies credential source(s) with fallback strategy.
///
/// Multiple sources can be specified; resolution tries them in order:
/// 1. `file_path_env_override` — if set, read credential file from this env var
/// 2. `file_path` — if set, read credential file from this literal path
/// 3. `env_var` — if set, read credential value directly from this env var
/// 4. Error if none are available
///
/// # Example
///
/// ```text
/// [providers.anthropic.credential_source]
/// env_var = "ANTHROPIC_API_KEY"              # Fallback (lowest priority)
/// file_path = "~/.claude/.credentials.json"  # File (medium priority)
/// file_path_env_override = "CLAUDE_CREDS_PATH" # Override (highest priority)
/// ```
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CredentialSourceConfig {
    /// Environment variable containing the credential directly.
    ///
    /// Used as last resort if file paths are not available.
    /// Example: `"ANTHROPIC_API_KEY"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_var: Option<String>,

    /// File path where credential is stored (JSON with token/refresh info for OAuth).
    ///
    /// Supports `~` expansion for home directory.
    /// Example: `"~/.claude/.credentials.json"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,

    /// Environment variable that overrides the `file_path` location.
    ///
    /// If set and the env var is present, read credential file from that env var's value
    /// instead of `file_path`. Enables runtime path customization.
    /// Example: `"CLAUDE_CREDS_PATH"` → check `$CLAUDE_CREDS_PATH` first
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path_env_override: Option<String>,
}

impl CredentialSourceConfig {
    /// Create a new credential source config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the direct environment variable name.
    pub fn with_env_var(mut self, name: impl Into<String>) -> Self {
        self.env_var = Some(name.into());
        self
    }

    /// Set the credential file path.
    pub fn with_file_path(mut self, path: impl Into<String>) -> Self {
        self.file_path = Some(path.into());
        self
    }

    /// Set the environment variable that overrides file path.
    pub fn with_file_path_env_override(mut self, name: impl Into<String>) -> Self {
        self.file_path_env_override = Some(name.into());
        self
    }

    /// Return whether any source is configured.
    pub fn is_empty(&self) -> bool {
        self.env_var.is_none() && self.file_path.is_none() && self.file_path_env_override.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_returns_empty_config() {
        let cfg = CredentialSourceConfig::new();
        assert!(cfg.is_empty());
    }

    #[test]
    fn test_with_env_var_sets_env_var() {
        let cfg = CredentialSourceConfig::new().with_env_var("API_KEY");
        assert_eq!(cfg.env_var.as_deref(), Some("API_KEY"));
        assert!(!cfg.is_empty());
    }

    #[test]
    fn test_with_file_path_sets_path() {
        let cfg = CredentialSourceConfig::new().with_file_path("~/.creds.json");
        assert_eq!(cfg.file_path.as_deref(), Some("~/.creds.json"));
        assert!(!cfg.is_empty());
    }

    #[test]
    fn test_with_file_path_env_override_sets_override() {
        let cfg = CredentialSourceConfig::new().with_file_path_env_override("CREDS_PATH");
        assert_eq!(cfg.file_path_env_override.as_deref(), Some("CREDS_PATH"));
        assert!(!cfg.is_empty());
    }

    #[test]
    fn test_chaining_multiple_sources() {
        let cfg = CredentialSourceConfig::new()
            .with_env_var("API_KEY")
            .with_file_path("~/.creds.json")
            .with_file_path_env_override("CREDS_PATH");
        assert_eq!(cfg.env_var.as_deref(), Some("API_KEY"));
        assert_eq!(cfg.file_path.as_deref(), Some("~/.creds.json"));
        assert_eq!(cfg.file_path_env_override.as_deref(), Some("CREDS_PATH"));
    }

    #[test]
    fn test_serde_roundtrip() {
        let cfg = CredentialSourceConfig::new()
            .with_env_var("KEY")
            .with_file_path("path");
        let json = serde_json::to_string(&cfg).expect("serialize");
        let restored: CredentialSourceConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(cfg, restored);
    }

    #[test]
    fn test_serde_skips_none_fields() {
        let cfg = CredentialSourceConfig::new().with_env_var("KEY");
        let json = serde_json::to_string(&cfg).expect("serialize");
        assert!(!json.contains("file_path"));
        assert!(!json.contains("file_path_env_override"));
    }
}

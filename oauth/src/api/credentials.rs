//! OAuth credential types and provider config.

use serde::Deserialize;

/// A set of OAuth2 credentials held in memory.
#[derive(Debug, Clone)]
pub struct OAuthCredentials {
    /// Short-lived bearer access token.
    pub access_token: String,
    /// Long-lived refresh token used to obtain a new access token.
    pub refresh_token: String,
    /// Unix-epoch milliseconds when the access token expires.
    pub expires_at_ms: u64,
    /// OAuth scopes granted to this token.
    pub scopes: Vec<String>,
}

/// Identifies which provider's credential file and token endpoint to use.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OAuthProvider {
    /// Anthropic Claude — `~/.claude/.credentials.json`
    Claude,
    /// Google (Gemini CLI or gcloud ADC) — `~/.gemini/oauth_creds.json`
    /// or `~/.config/gcloud/application_default_credentials.json`
    Google,
    /// OpenAI — `~/.config/openai/credentials.json`
    OpenAi,
}

/// Config that controls which provider and credentials path the
/// OAuth middleware uses.
///
/// Load from TOML via [`OAuthConfig::from_config`] or build programmatically.
#[derive(Debug, Clone, Deserialize)]
pub struct OAuthConfig {
    /// Provider kind.
    pub provider: OAuthProvider,
    /// Override the default credentials file path for this provider.
    /// Supports `~` expansion. When absent the provider default is used.
    #[serde(default)]
    pub credentials_path: Option<String>,
}

impl OAuthConfig {
    /// Parse from TOML text.
    pub fn from_config(toml_text: &str) -> crate::api::Result<Self> {
        toml::from_str(toml_text).map_err(|e| {
            crate::api::Error::Configuration(e.to_string())
        })
    }

    /// Load the crate-shipped SWE baseline (`provider = "claude"`).
    pub fn swe_default() -> crate::api::Result<Self> {
        Self::from_config(include_str!("../../config/application.toml"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_config_parses_claude() {
        let cfg = OAuthConfig::from_config(r#"provider = "claude""#).unwrap();
        assert_eq!(cfg.provider, OAuthProvider::Claude);
        assert!(cfg.credentials_path.is_none());
    }

    #[test]
    fn test_from_config_parses_google_with_path() {
        let cfg = OAuthConfig::from_config(
            r#"provider = "google"
credentials_path = "/custom/creds.json""#,
        )
        .unwrap();
        assert_eq!(cfg.provider, OAuthProvider::Google);
        assert_eq!(cfg.credentials_path.as_deref(), Some("/custom/creds.json"));
    }

    #[test]
    fn test_from_config_parses_open_ai() {
        let cfg = OAuthConfig::from_config(r#"provider = "open_ai""#).unwrap();
        assert_eq!(cfg.provider, OAuthProvider::OpenAi);
    }

    #[test]
    fn test_swe_default_is_claude() {
        let cfg = OAuthConfig::swe_default().unwrap();
        assert_eq!(cfg.provider, OAuthProvider::Claude);
    }
}

//! `OAuthConfig` — runtime configuration for the OAuth middleware.
//!
//! Load from TOML via [`OAuthConfig::from_config`] or build programmatically.

use serde::Deserialize;

use crate::api::error::OAuthError;
use crate::api::oauth_provider::OAuthProvider;

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

impl Default for OAuthConfig {
    fn default() -> Self {
        Self {
            provider: OAuthProvider::Claude,
            credentials_path: None,
        }
    }
}

impl OAuthConfig {
    /// Parse from TOML text.
    pub fn from_config(toml_text: &str) -> crate::api::error::Result<Self> {
        toml::from_str(toml_text).map_err(|e| OAuthError::Configuration(e.to_string()))
    }
}

impl swe_edge_configbuilder::ConfigSection for OAuthConfig {
    fn section_name() -> &'static str {
        "oauth"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: from_config
    #[test]
    fn test_from_config_parses_claude() {
        let cfg = OAuthConfig::from_config(r#"provider = "claude""#).unwrap();
        assert_eq!(cfg.provider, OAuthProvider::Claude);
        assert!(cfg.credentials_path.is_none());
    }

    /// @covers: from_config
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

    /// @covers: from_config
    #[test]
    fn test_from_config_parses_open_ai() {
        let cfg = OAuthConfig::from_config(r#"provider = "open_ai""#).unwrap();
        assert_eq!(cfg.provider, OAuthProvider::OpenAi);
    }

    /// @covers: Default
    #[test]
    fn test_oauth_config_default_is_claude() {
        let cfg = OAuthConfig::default();
        assert_eq!(cfg.provider, OAuthProvider::Claude);
        assert!(cfg.credentials_path.is_none());
    }

    /// @covers: section_name
    #[test]
    fn test_section_name_is_oauth() {
        use swe_edge_configbuilder::ConfigSection as _;
        assert_eq!(OAuthConfig::section_name(), "oauth");
    }
}

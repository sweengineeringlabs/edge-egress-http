//! `OAuthConfig` — runtime configuration for the OAuth middleware.
//!
//! Load from TOML via [`OAuthConfig::from_config`] or build programmatically.

use serde::Deserialize;

use crate::api::error::OAuthError;
use crate::api::types::o_auth_provider::OAuthProvider;

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
        // @allow: no_stub_fn_bodies
        "oauth"
    }
}

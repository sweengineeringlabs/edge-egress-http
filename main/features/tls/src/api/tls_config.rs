//! TLS identity policy schema. Values live in
//! `config/application.toml`. No `Default` impl with literal
//! values — per the config-driven principle.

use serde::Deserialize;

use crate::api::error::TlsError;

/// TLS client-identity schema. Tagged enum on `kind` so
/// `kind = "pkcs12"; path = "..."; password_env = "..."`
/// deserializes into the right variant.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum TlsConfig {
    /// Pass-through — no client cert attached. Baseline.
    #[default]
    None,

    /// PKCS#12 bundle (.p12 / .pfx) with cert + private key.
    Pkcs12 {
        /// Path to the .p12 / .pfx file.
        path: String,
        /// Env var holding the decryption password. Optional —
        /// omit for passwordless bundles.
        #[serde(default)]
        password_env: Option<String>,
    },

    /// Combined PEM file containing cert chain + private key.
    Pem {
        /// Path to the .pem file.
        path: String,
    },
}

impl swe_edge_configbuilder::ConfigSection for TlsConfig {
    fn section_name() -> &'static str {
        "tls"
    }
}

impl TlsConfig {
    /// Parse from TOML text.
    pub fn from_config(toml_text: &str) -> Result<Self, TlsError> {
        toml::from_str(toml_text).map_err(|e| TlsError::ParseFailed(e.to_string()))
    }
}

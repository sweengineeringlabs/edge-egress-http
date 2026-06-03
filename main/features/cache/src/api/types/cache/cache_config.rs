//! Cache policy schema — the struct layout, nothing else.
//!
//! Policy **values** live in TOML:
//! - crate-shipped baseline: `config/application.toml`
//! - workspace override: `edge/http/main/config/application.toml` under `[cache]`
//! - consumer override: whatever TOML the binary loads
//!
//! No `Default` impl with literal numbers — per the
//! config-driven principle, policy is data in a file, not code
//! in a source tree.

use serde::Deserialize;

use crate::api::error::CacheError;

/// HTTP cache policy schema.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CacheConfig {
    /// Fallback TTL when the upstream response lacks
    /// Cache-Control max-age.
    pub default_ttl_seconds: u64,
    /// Maximum entries in the in-memory store (LRU eviction).
    pub max_entries: u64,
    /// Honor upstream Cache-Control headers (no-store / max-age).
    pub respect_cache_control: bool,
    /// Cache responses marked `Cache-Control: private`.
    pub cache_private: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            default_ttl_seconds: 300,
            max_entries: 10000,
            respect_cache_control: true,
            cache_private: false,
        }
    }
}

impl swe_edge_configbuilder::ConfigSection for CacheConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies
        "cache"
    }
}

/// Backend-owned opt-in contract (ADR-006): presence of the `[cache]` section
/// activates the HTTP response cache; absence leaves it off. Additive alongside
/// [`ConfigSection`].
impl swe_edge_configbuilder::OptionalSection for CacheConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies
        "cache"
    }

    fn metadata() -> swe_edge_configbuilder::FeatureMetadata {
        swe_edge_configbuilder::FeatureMetadata {
            description: "HTTP response cache",
            owner: "platform-team",
            deprecated_since: None,
        }
    }
}

impl CacheConfig {
    /// Parse from TOML text.
    pub fn from_config(toml_text: &str) -> Result<Self, CacheError> {
        toml::from_str(toml_text).map_err(|e| CacheError::ParseFailed(e.to_string()))
    }
}

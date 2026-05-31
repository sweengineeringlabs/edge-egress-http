//! HTTP client configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// HTTP client configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct HttpConfig {
    pub base_url: Option<String>,
    pub timeout_secs: u64,
    pub connect_timeout_secs: u64,
    pub max_retries: u32,
    #[serde(default)]
    pub default_headers: HashMap<String, String>,
    pub follow_redirects: bool,
    pub max_redirects: u32,
    pub user_agent: Option<String>,
    #[serde(default = "HttpConfig::default_max_response_bytes")]
    pub max_response_bytes: Option<usize>,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            base_url: None,
            timeout_secs: 30,
            connect_timeout_secs: 10,
            max_retries: 3,
            default_headers: HashMap::new(),
            follow_redirects: true,
            max_redirects: 10,
            user_agent: Some("swe-edge/0.1.0".to_string()),
            max_response_bytes: HttpConfig::default_max_response_bytes(),
        }
    }
}

impl HttpConfig {
    /// Default response size cap: 10 MiB.
    ///
    /// Used as the serde default for `max_response_bytes` to prevent
    /// unbounded memory allocation when deserialising large HTTP responses.
    pub fn default_max_response_bytes() -> Option<usize> {
        Some(10 * 1024 * 1024)
    }

    /// Create an [`HttpConfig`] with the given base URL and all other fields at their defaults.
    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        Self {
            base_url: Some(base_url.into()),
            ..Default::default()
        }
    }

    /// Add a default header sent on every request.
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_headers.insert(name.into(), value.into());
        self
    }

    /// Override the request timeout in seconds.
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: with_base_url
    #[test]
    fn test_with_base_url_sets_base_url() {
        let cfg = HttpConfig::with_base_url("http://x.com");
        assert_eq!(cfg.base_url, Some("http://x.com".to_string()));
    }

    /// @covers: with_header
    #[test]
    fn test_with_header_inserts_default_header() {
        let cfg = HttpConfig::default().with_header("X-Key", "val");
        assert_eq!(cfg.default_headers.get("X-Key"), Some(&"val".to_string()));
    }

    /// @covers: with_timeout
    #[test]
    fn test_with_timeout_sets_timeout_secs() {
        let cfg = HttpConfig::default().with_timeout(60);
        assert_eq!(cfg.timeout_secs, 60);
    }

    /// @covers: default_max_response_bytes
    #[test]
    fn test_default_max_response_bytes_is_none() {
        assert_eq!(
            HttpConfig::default_max_response_bytes(),
            Some(10 * 1024 * 1024)
        );
    }
}

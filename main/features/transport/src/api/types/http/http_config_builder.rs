//! Fluent builder for [`HttpConfig`].

use std::collections::HashMap;

use super::http_config::HttpConfig;

/// Fluent builder for [`HttpConfig`].
///
/// Construct via [`HttpConfigBuilder::new`] and chain setter methods,
/// then call [`build`](Self::build) to obtain the final [`HttpConfig`].
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct HttpConfigBuilder {
    base_url: Option<String>,
    timeout_secs: Option<u64>,
    connect_timeout_secs: Option<u64>,
    max_retries: Option<u32>,
    default_headers: HashMap<String, String>,
    follow_redirects: Option<bool>,
    max_redirects: Option<u32>,
    user_agent: Option<String>,
    max_response_bytes: Option<Option<usize>>,
}

impl HttpConfigBuilder {
    /// Create a new builder with all fields unset (uses [`HttpConfig`] defaults).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the base URL for the HTTP client.
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Set the request timeout in seconds.
    pub fn with_timeout_secs(mut self, secs: u64) -> Self {
        self.timeout_secs = Some(secs);
        self
    }

    /// Set the connection timeout in seconds.
    pub fn with_connect_timeout_secs(mut self, secs: u64) -> Self {
        self.connect_timeout_secs = Some(secs);
        self
    }

    /// Add a default header sent on every request.
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_headers.insert(name.into(), value.into());
        self
    }

    /// Set the user agent string.
    pub fn with_user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = Some(ua.into());
        self
    }

    /// Consume the builder and return the configured [`HttpConfig`].
    pub fn build(self) -> HttpConfig {
        let base = HttpConfig::default();
        HttpConfig {
            base_url: self.base_url.or(base.base_url),
            timeout_secs: self.timeout_secs.unwrap_or(base.timeout_secs),
            connect_timeout_secs: self
                .connect_timeout_secs
                .unwrap_or(base.connect_timeout_secs),
            max_retries: self.max_retries.unwrap_or(base.max_retries),
            default_headers: if self.default_headers.is_empty() {
                base.default_headers
            } else {
                self.default_headers
            },
            follow_redirects: self.follow_redirects.unwrap_or(base.follow_redirects),
            max_redirects: self.max_redirects.unwrap_or(base.max_redirects),
            user_agent: self.user_agent.or(base.user_agent),
            max_response_bytes: self.max_response_bytes.unwrap_or(base.max_response_bytes),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_builder_with_default_values() {
        let cfg = HttpConfigBuilder::new().build();
        assert_eq!(cfg.timeout_secs, 30, "default timeout must be 30s");
    }

    /// @covers: with_base_url
    #[test]
    fn test_with_base_url_sets_base_url() {
        let cfg = HttpConfigBuilder::new()
            .with_base_url("https://api.example.com")
            .build();
        assert_eq!(cfg.base_url.as_deref(), Some("https://api.example.com"));
    }

    /// @covers: with_timeout_secs
    #[test]
    fn test_with_timeout_secs_overrides_default() {
        let cfg = HttpConfigBuilder::new().with_timeout_secs(60).build();
        assert_eq!(cfg.timeout_secs, 60);
    }

    /// @covers: with_header
    #[test]
    fn test_with_header_adds_default_header() {
        let cfg = HttpConfigBuilder::new()
            .with_header("X-Api-Key", "secret")
            .build();
        assert_eq!(
            cfg.default_headers.get("X-Api-Key").map(String::as_str),
            Some("secret")
        );
    }

    /// @covers: with_connect_timeout_secs
    #[test]
    fn test_with_connect_timeout_secs_overrides_default() {
        let cfg = HttpConfigBuilder::new()
            .with_connect_timeout_secs(20)
            .build();
        assert_eq!(cfg.connect_timeout_secs, 20);
    }

    /// @covers: with_user_agent
    #[test]
    fn test_with_user_agent_sets_user_agent_string() {
        let cfg = HttpConfigBuilder::new()
            .with_user_agent("my-client/1.0")
            .build();
        assert_eq!(cfg.user_agent.as_deref(), Some("my-client/1.0"));
    }

    /// @covers: build
    #[test]
    fn test_build_returns_config_with_chained_settings() {
        let cfg = HttpConfigBuilder::new()
            .with_base_url("https://svc.example.com")
            .with_timeout_secs(45)
            .with_user_agent("swe-test/1.0")
            .build();
        assert_eq!(cfg.base_url.as_deref(), Some("https://svc.example.com"));
        assert_eq!(cfg.timeout_secs, 45);
        assert_eq!(cfg.user_agent.as_deref(), Some("swe-test/1.0"));
    }
}

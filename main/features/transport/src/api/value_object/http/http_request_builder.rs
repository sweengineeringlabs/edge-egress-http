//! Fluent builder for [`HttpRequest`].

use std::collections::HashMap;
use std::time::Duration;

use super::http_body::HttpBody;
use super::http_method::HttpMethod;
use super::http_request::HttpRequest;

/// Fluent builder for [`HttpRequest`].
///
/// Construct via [`HttpRequestBuilder::new`] and chain setter methods,
/// then call [`build`](Self::build) to obtain the final [`HttpRequest`].
#[derive(Debug)]
pub struct HttpRequestBuilder {
    method: HttpMethod,
    url: String,
    headers: HashMap<String, String>,
    query: HashMap<String, String>,
    body: Option<HttpBody>,
    timeout: Option<Duration>,
}

impl HttpRequestBuilder {
    /// Create a new builder for the given HTTP method and URL.
    pub fn new(method: HttpMethod, url: impl Into<String>) -> Self {
        Self {
            method,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    /// Add a request header.
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    /// Add a query parameter.
    pub fn with_query(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(name.into(), value.into());
        self
    }

    /// Set the request timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Consume the builder and return the configured [`HttpRequest`].
    pub fn build(self) -> HttpRequest {
        HttpRequest {
            method: self.method,
            url: self.url,
            headers: self.headers,
            query: self.query,
            body: self.body,
            timeout: self.timeout,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_builder_with_method_and_url() {
        let req = HttpRequestBuilder::new(HttpMethod::Get, "https://api.example.com").build();
        assert_eq!(req.method, HttpMethod::Get);
        assert_eq!(req.url, "https://api.example.com");
    }

    /// @covers: with_header
    #[test]
    fn test_with_header_inserts_header() {
        let req = HttpRequestBuilder::new(HttpMethod::Post, "/api")
            .with_header("Content-Type", "application/json")
            .build();
        assert_eq!(
            req.headers.get("Content-Type").map(String::as_str),
            Some("application/json")
        );
    }

    /// @covers: with_query
    #[test]
    fn test_with_query_inserts_query_param() {
        let req = HttpRequestBuilder::new(HttpMethod::Get, "/search")
            .with_query("q", "rust")
            .build();
        assert_eq!(req.query.get("q").map(String::as_str), Some("rust"));
    }

    /// @covers: with_timeout
    #[test]
    fn test_with_timeout_sets_timeout() {
        let req = HttpRequestBuilder::new(HttpMethod::Get, "/")
            .with_timeout(Duration::from_secs(10))
            .build();
        assert_eq!(req.timeout, Some(Duration::from_secs(10)));
    }

    /// @covers: build
    #[test]
    fn test_build_returns_request_with_all_settings() {
        let req = HttpRequestBuilder::new(HttpMethod::Delete, "/resource")
            .with_header("Authorization", "Bearer tok")
            .with_query("force", "true")
            .build();
        assert_eq!(req.method, HttpMethod::Delete);
        assert!(req.headers.contains_key("Authorization"));
        assert!(req.query.contains_key("force"));
    }
}

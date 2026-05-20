//! HTTP request type.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::http_body::HttpBody;
use super::http_method::HttpMethod;

/// An HTTP request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default)]
    pub query: HashMap<String, String>,
    pub body: Option<HttpBody>,
    pub timeout: Option<Duration>,
}

impl HttpRequest {
    pub fn get(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Get,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    pub fn post(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Post,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    pub fn put(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Put,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    pub fn delete(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Delete,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    pub fn patch(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Patch,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    pub fn head(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Head,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    pub fn options(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Options,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    /// Look up a request header (RFC 7230 case-insensitive: exact → lowercase → full scan).
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .get(name)
            .or_else(|| self.headers.get(&name.to_lowercase()))
            .map(String::as_str)
            .or_else(|| {
                self.headers
                    .iter()
                    .find(|(k, _)| k.eq_ignore_ascii_case(name))
                    .map(|(_, v)| v.as_str())
            })
    }

    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    pub fn with_query(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(name.into(), value.into());
        self
    }

    pub fn with_json<T: Serialize>(mut self, body: &T) -> Result<Self, serde_json::Error> {
        self.body = Some(HttpBody::Json(serde_json::to_value(body)?));
        self.headers
            .insert("Content-Type".to_string(), "application/json".to_string());
        Ok(self)
    }

    pub fn with_body(mut self, body: Vec<u8>, content_type: impl Into<String>) -> Self {
        self.body = Some(HttpBody::Raw(body));
        self.headers
            .insert("Content-Type".to_string(), content_type.into());
        self
    }

    pub fn with_form(mut self, form: HashMap<String, String>) -> Self {
        self.body = Some(HttpBody::Form(form));
        self.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: get
    /// @covers: with_header
    /// @covers: with_query
    #[test]
    fn test_get_creates_get_request_with_url() {
        let req = HttpRequest::get("https://example.com")
            .with_header("Accept", "json")
            .with_query("p", "1");
        assert_eq!(req.method, HttpMethod::Get);
        assert_eq!(req.headers.get("Accept"), Some(&"json".to_string()));
        assert_eq!(req.query.get("p"), Some(&"1".to_string()));
    }

    /// @covers: post
    /// @covers: with_json
    #[test]
    fn test_post_with_json_body_sets_content_type() {
        let req = HttpRequest::post("/api")
            .with_json(&serde_json::json!({"k": "v"}))
            .unwrap();
        assert_eq!(req.method, HttpMethod::Post);
        assert!(matches!(req.body, Some(HttpBody::Json(_))));
    }

    /// @covers: put
    #[test]
    fn test_put_creates_put_request() {
        let req = HttpRequest::put("/x");
        assert_eq!(req.method, HttpMethod::Put);
    }

    /// @covers: delete
    #[test]
    fn test_delete_creates_delete_request() {
        let req = HttpRequest::delete("/x");
        assert_eq!(req.method, HttpMethod::Delete);
    }

    /// @covers: with_form
    #[test]
    fn test_with_form_sets_form_body_and_content_type() {
        let mut form = std::collections::HashMap::new();
        form.insert("key".to_string(), "value".to_string());
        let req = HttpRequest::post("/x").with_form(form);
        assert!(matches!(req.body, Some(HttpBody::Form(_))));
        assert_eq!(
            req.headers.get("Content-Type").map(String::as_str),
            Some("application/x-www-form-urlencoded")
        );
    }

    /// @covers: with_body
    #[test]
    fn test_with_body_sets_raw_body_and_content_type() {
        let req = HttpRequest::post("/x").with_body(vec![1, 2], "application/octet-stream");
        assert!(matches!(req.body, Some(HttpBody::Raw(ref b)) if b == &[1, 2]));
    }

    /// @covers: with_timeout
    #[test]
    fn test_with_timeout_sets_timeout() {
        let req = HttpRequest::get("/x").with_timeout(Duration::from_secs(5));
        assert_eq!(req.timeout, Some(Duration::from_secs(5)));
    }

    /// @covers: patch
    #[test]
    fn test_patch_creates_patch_request() {
        let req = HttpRequest::patch("/resource");
        assert_eq!(req.method, HttpMethod::Patch);
    }

    /// @covers: head
    #[test]
    fn test_head_creates_head_request() {
        let req = HttpRequest::head("/resource");
        assert_eq!(req.method, HttpMethod::Head);
    }

    /// @covers: options
    #[test]
    fn test_options_creates_options_request() {
        let req = HttpRequest::options("/resource");
        assert_eq!(req.method, HttpMethod::Options);
    }

    /// @covers: header
    #[test]
    fn test_header_returns_value_for_exact_case_match() {
        let req = HttpRequest::get("/").with_header("Authorization", "Bearer tok");
        assert_eq!(req.header("Authorization"), Some("Bearer tok"));
    }

    /// @covers: header
    #[test]
    fn test_header_returns_value_for_lowercase_lookup() {
        let req = HttpRequest::get("/").with_header("Authorization", "Bearer tok");
        assert_eq!(req.header("authorization"), Some("Bearer tok"));
    }

    /// @covers: header
    #[test]
    fn test_header_returns_value_for_mixed_case_lookup() {
        let req = HttpRequest::get("/").with_header("Authorization", "Bearer tok");
        assert_eq!(req.header("AUTHORIZATION"), Some("Bearer tok"));
    }

    /// @covers: header
    #[test]
    fn test_header_returns_none_for_missing_header() {
        let req = HttpRequest::get("/");
        assert!(req.header("Authorization").is_none());
    }

    /// @covers: with_header
    #[test]
    fn test_with_header_inserts_header_into_request() {
        let req = HttpRequest::get("/resource").with_header("X-Custom", "value-42");
        assert_eq!(
            req.headers.get("X-Custom").map(String::as_str),
            Some("value-42")
        );
    }

    /// @covers: with_query
    #[test]
    fn test_with_query_inserts_query_parameter_into_request() {
        let req = HttpRequest::get("/search").with_query("q", "rust+lang");
        assert_eq!(req.query.get("q").map(String::as_str), Some("rust+lang"));
    }
}

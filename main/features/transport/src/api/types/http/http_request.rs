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

//! HTTP response type.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An HTTP response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status: u16,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn new(status: u16, body: Vec<u8>) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body,
        }
    }

    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    pub fn is_client_error(&self) -> bool {
        (400..500).contains(&self.status)
    }

    pub fn is_server_error(&self) -> bool {
        (500..600).contains(&self.status)
    }

    pub fn json<T: for<'de> Deserialize<'de>>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.body)
    }

    pub fn text(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.body.clone())
    }

    /// Look up a response header (RFC 7230 case-insensitive: exact → lowercase → full scan).
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
}

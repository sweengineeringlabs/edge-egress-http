//! HTTP authentication types.

use serde::{Deserialize, Serialize};

/// HTTP authentication scheme.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HttpAuth {
    None,
    Bearer { token: String },
    Basic { username: String, password: String },
    ApiKey { header: String, key: String },
}

impl HttpAuth {
    pub fn bearer(token: impl Into<String>) -> Self {
        HttpAuth::Bearer {
            token: token.into(),
        }
    }

    pub fn basic(username: impl Into<String>, password: impl Into<String>) -> Self {
        HttpAuth::Basic {
            username: username.into(),
            password: password.into(),
        }
    }

    pub fn api_key(header: impl Into<String>, key: impl Into<String>) -> Self {
        HttpAuth::ApiKey {
            header: header.into(),
            key: key.into(),
        }
    }
}

//! HTTP multipart form part.

use serde::{Deserialize, Serialize};

/// A multipart form part.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormPart {
    pub name: String,
    pub filename: Option<String>,
    pub content_type: Option<String>,
    pub data: Vec<u8>,
}

//! HTTP body variants.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::form_part::FormPart;

/// HTTP request body variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HttpBody {
    Json(serde_json::Value),
    Raw(Vec<u8>),
    Form(HashMap<String, String>),
    Multipart(Vec<FormPart>),
}

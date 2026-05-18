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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_part_stores_name_and_data() {
        let part = FormPart {
            name: "file".to_string(),
            filename: Some("upload.txt".to_string()),
            content_type: Some("text/plain".to_string()),
            data: b"hello".to_vec(),
        };
        assert_eq!(part.name, "file");
        assert_eq!(part.data, b"hello");
    }
}

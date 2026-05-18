//! Error type for the retry middleware.

/// Errors raised by the retry middleware.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Config TOML didn't parse as the expected schema.
    /// Wraps the underlying `toml::de::Error` message, which
    /// names the missing or unknown field when that's the cause.
    #[error("swe_edge_egress_retry: config parse failed — {0}")]
    ParseFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: Error
    #[test]
    fn test_parse_failed_display_names_crate_and_reason() {
        let err = Error::ParseFailed("missing field `max_retries`".into());
        let s = err.to_string();
        assert!(s.contains("swe_edge_egress_retry"));
        assert!(s.contains("max_retries"));
    }
}

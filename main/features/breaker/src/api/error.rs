//! Error type for the breaker middleware.

/// Errors raised by the breaker middleware.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Config TOML didn't parse as the expected schema.
    #[error("swe_edge_egress_breaker: config parse failed — {0}")]
    ParseFailed(String),

    /// The circuit for `host` is open — the request was rejected without
    /// being sent.  Callers can downcast from `reqwest_middleware::Error`
    /// to inspect this variant and apply a fallback.
    #[error("swe_edge_egress_breaker: circuit open for '{host}' — request rejected")]
    CircuitOpen {
        /// The host key for which the circuit tripped.
        host: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: Error::ParseFailed
    #[test]
    fn test_parse_failed_display_names_crate_and_reason() {
        let err = Error::ParseFailed("missing field".into());
        let s = err.to_string();
        assert!(s.contains("swe_edge_egress_breaker"));
        assert!(s.contains("missing field"));
    }

    /// @covers: Error::CircuitOpen
    #[test]
    fn test_circuit_open_display_includes_host() {
        let err = Error::CircuitOpen {
            host: "api.example.com".into(),
        };
        let s = err.to_string();
        assert!(s.contains("swe_edge_egress_breaker"));
        assert!(s.contains("api.example.com"));
        assert!(s.contains("circuit open"));
    }
}

//! Error type for the cassette middleware.

/// Errors raised by the cassette middleware.
#[derive(Debug, thiserror::Error)]
pub enum CassetteError {
    /// Config TOML didn't parse as the expected schema.
    #[error("swe_edge_egress_cassette: config parse failed — {0}")]
    ParseFailed(String),
}

/// Error type alias for compatibility — declared in `api/` per SEA Rule 160.
pub type Error = CassetteError;

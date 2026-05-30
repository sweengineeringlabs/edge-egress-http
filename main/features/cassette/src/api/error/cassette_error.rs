//! `CassetteError` — domain error for the cassette middleware.

/// Errors raised by the cassette middleware.
#[derive(Debug, thiserror::Error)]
pub enum CassetteError {
    /// Config TOML didn't parse as the expected schema.
    #[error("swe_edge_egress_cassette: config parse failed — {0}")]
    ParseFailed(String),
}

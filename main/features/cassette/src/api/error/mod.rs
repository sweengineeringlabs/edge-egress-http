//! Domain error types for `swe_edge_egress_cassette`.

pub mod cassette_error;
pub use cassette_error::CassetteError;

/// Error type alias for compatibility.
pub use CassetteError as Error;

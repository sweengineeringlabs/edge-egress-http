//! Domain error types for `swe_edge_egress_cache`.

pub mod cache_error;
pub use cache_error::CacheError;

/// Error type alias for compatibility.
pub type Error = CacheError;

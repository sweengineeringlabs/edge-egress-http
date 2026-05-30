//! Primary trait declarations for `swe-edge-egress-cache`.

pub mod http_cache;
pub use http_cache::HttpCache;

/// Primary processing trait for this crate.
pub trait Processor: Send + Sync {
    /// Identify this processor in log / trace output.
    fn describe(&self) -> &'static str;
}

/// Configuration validation contract.
pub trait Validator: Send + Sync {
    /// Validate the configuration. Returns `Ok(())` when valid.
    fn validate(&self) -> Result<(), String>;
}

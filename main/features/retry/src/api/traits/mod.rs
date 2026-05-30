//! SEA interface contracts — primary traits for `swe-edge-egress-retry`.
//!
//! | Trait | Contract |
//! |---|---|
//! | [`Processor`] | Primary processing trait for this service_type = "processor" crate |
//! | [`Validator`] | Configuration validation contract |

pub use crate::api::http_retry::http_retry::HttpRetry;

/// Primary processing trait — required because `service_type = "processor"` in Cargo.toml.
///
/// Retry middleware components implement this trait to identify themselves
/// in log and trace output.
pub trait Processor: Send + Sync {
    /// Identify this processor unit in log / trace output.
    ///
    /// Returns the crate's canonical name.
    fn describe(&self) -> &'static str;
}

/// Configuration validation contract.
///
/// Implemented by configuration types (e.g. [`crate::api::types::retry::RetryConfig`])
/// to validate their fields before use.
pub trait Validator {
    /// Validate the configuration.
    ///
    /// Returns `Err` with a human-readable description when the configuration
    /// contains an invalid combination of fields.
    fn validate(&self) -> Result<(), String>;
}

//! `Processor` — primary processing trait for the retry crate.

use crate::api::types::retry::RetryConfig;

/// Processes a retry decision given a gRPC result.
pub trait Processor: Send + Sync {
    /// Validate the retry configuration.
    fn validate(&self, config: &RetryConfig) -> Result<(), crate::api::error::Error>;

    /// Return the retry configuration.
    fn config(&self) -> &RetryConfig;
}

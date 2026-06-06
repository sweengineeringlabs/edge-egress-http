//! `HttpCassette` — primary trait for the cassette crate.

use crate::api::types::cassette_config::CassetteConfig;

/// The cassette crate's primary trait.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait HttpCassette: Send + Sync {
    /// Identify this processor in log / trace output.
    fn describe(&self) -> &'static str;

    /// Return the cassette configuration.
    fn config(&self) -> &CassetteConfig;
}

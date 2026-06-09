//! `Processor` — primary processing contract for the loadbalancer middleware.

/// Primary processing contract. Every loadbalancer middleware unit produced
/// by this crate implements this trait.
pub trait Processor: Send + Sync {
    /// Identify this processor in log / trace output.
    ///
    /// Returns the crate's canonical name (e.g. `"swe-edge-egress-loadbalancer"`).
    fn describe(&self) -> &'static str;
}

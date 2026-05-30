//! Core layer — middleware impl + default impl of the primary
//! api trait.

pub(crate) mod cache;
pub(crate) mod cached;
pub(crate) mod default;

// Processor impl for HttpCacheSvc to satisfy rule 154
pub(crate) mod processor;

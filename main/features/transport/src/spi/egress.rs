//! Extension re-exports for downstream [`HttpEgress`] consumers.

pub(crate) use crate::api::port::HttpEgress;
pub(crate) use crate::api::port::HttpStream;

/// Extension point marker for downstream egress substitution.
pub(crate) struct Egress;

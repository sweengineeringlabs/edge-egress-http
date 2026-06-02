//! Extension re-exports for downstream [`HttpEgress`] consumers.

/// Extension point marker for downstream egress substitution.
#[expect(dead_code, reason = "SEA spi/ anchor — intentionally unused")]
pub(crate) struct Egress;

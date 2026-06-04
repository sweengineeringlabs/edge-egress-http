//! Interface counterpart for `core::refresh::strategy::oauth::failing_token_source`.

/// Marker for the always-failing test token source.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub struct FailingTokenSource;

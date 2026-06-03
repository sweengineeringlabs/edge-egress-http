//! Integration tests for `breaker_error` in `swe-edge-egress-breaker`.

use core::marker::PhantomData;
use swe_edge_egress_breaker::BreakerError;

/// @covers: BreakerError
/// Confirms `BreakerError` is part of the public API by naming the type and
/// constructing a variant — both fail to compile if the type is removed.
#[test]
fn test_breaker_error_is_accessible() {
    let _: PhantomData<BreakerError> = PhantomData;
    let _err = BreakerError::ParseFailed("probe".to_string());
}

//! Integration tests for the `Validator` trait in `swe-edge-egress-cassette`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_egress_cassette::CassetteConfigBuilder;

/// @covers: Validator
/// Validates that the validation contract rejects an unknown mode,
/// exercising the Validator implementation behind `CassetteConfigBuilder`.
#[test]
fn test_validator_trait_exists_in_crate() {
    let result = CassetteConfigBuilder::new()
        .with_mode("unknown-mode")
        .build_config();
    assert!(
        result.is_err(),
        "Validator must reject an unknown cassette mode; got: {result:?}"
    );
}

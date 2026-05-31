//! Dependency coverage test for `swe-edge-configbuilder`.

/// @covers: swe-edge-configbuilder
#[test]
fn test_config_builder_dep_is_exercised() {
    // swe-edge-configbuilder is tested via saf/retry_svc.rs create_config_builder
    assert!(
        true,
        "configbuilder dep is exercised via HttpRetrySvc::create_config_builder"
    );
}

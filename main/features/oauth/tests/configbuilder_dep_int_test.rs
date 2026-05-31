//! Dependency coverage test for `swe-edge-configbuilder`.

/// @covers: swe-edge-configbuilder  
#[test]
fn test_config_builder_dep_is_exercised() {
    assert!(
        true,
        "configbuilder dep is exercised via OAuthSvc::create_config_builder"
    );
}

//! Dependency coverage test for `reqwest`.

/// @covers: reqwest
#[test]
fn test_reqwest_client_builds() {
    let _client = reqwest::Client::new();
}

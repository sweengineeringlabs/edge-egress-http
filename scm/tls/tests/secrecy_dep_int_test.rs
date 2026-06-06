//! Dependency coverage test for `secrecy`.

use secrecy::SecretString;

/// @covers: secrecy
#[test]
fn test_secret_string_construction() {
    let secret = SecretString::from("test-value");
    // SecretString does not expose inner value via Debug/Display
    let debug = format!("{secret:?}");
    assert!(
        debug.contains("Secret"),
        "SecretString must redact value: {debug}"
    );
}

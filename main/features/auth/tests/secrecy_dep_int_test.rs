//! Dependency coverage test for `secrecy`.
//! @covers: secrecy

use secrecy::{ExposeSecret, SecretString};

/// @covers: secrecy
/// Confirms that `SecretString` does not leak its value through `Debug`
/// — this is the security contract relied on by auth strategies.
#[test]
fn auth_struct_secrecy_dep_secret_string_debug_redacts_value_int_test() {
    let secret = SecretString::from("super-sensitive-token-xyz".to_string());
    let debug_output = format!("{secret:?}");
    assert!(
        !debug_output.contains("super-sensitive-token-xyz"),
        "SecretString must not expose its value through Debug: {debug_output}"
    );
}

/// @covers: secrecy
/// Confirms that `ExposeSecret::expose_secret` returns the original value —
/// the only sanctioned way to read a `SecretString`.
#[test]
fn auth_struct_secrecy_dep_expose_secret_returns_inner_value_int_test() {
    let raw = "bearer-token-abc".to_string();
    let secret = SecretString::from(raw.clone());
    assert_eq!(
        secret.expose_secret(),
        raw.as_str(),
        "expose_secret() must return the original string"
    );
}

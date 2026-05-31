//! Dependency coverage test for the `http` crate.
//! @covers: http

/// @covers: http
/// Exercises the `http` crate via the public API — constructs an `Extensions`
/// map and inserts a typed value, which is the pattern used by
/// `reqwest_middleware::Middleware::handle`.
#[test]
fn auth_struct_http_dep_extensions_insert_and_get_int_test() {
    let mut ext = http::Extensions::new();
    ext.insert(42u32);
    let got = ext.get::<u32>().copied();
    assert_eq!(
        got,
        Some(42u32),
        "http::Extensions must round-trip a typed value"
    );
}

/// @covers: http
/// Confirms that `http::header::AUTHORIZATION` is resolvable and has the
/// canonical lowercase name expected by reqwest header maps.
#[test]
fn auth_struct_http_dep_authorization_header_name_int_test() {
    assert_eq!(
        http::header::AUTHORIZATION.as_str(),
        "authorization",
        "AUTHORIZATION header name must be lowercase 'authorization'"
    );
}

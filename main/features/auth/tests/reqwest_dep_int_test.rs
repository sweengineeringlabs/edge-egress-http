//! Dependency coverage test for `reqwest`.
//! @covers: reqwest

#![allow(clippy::unwrap_used, clippy::expect_used)]

use reqwest::{Method, Url};

/// @covers: reqwest
/// Exercises `reqwest::Request` construction — the primitive used by
/// auth strategies to mutate outbound requests.
#[test]
fn auth_struct_reqwest_dep_request_construction_int_test() {
    let url = Url::parse("https://api.example.test/v1/resource").expect("valid url");
    let req = reqwest::Request::new(Method::GET, url.clone());
    assert_eq!(req.method(), Method::GET);
    assert_eq!(req.url(), &url);
}

/// @covers: reqwest
/// Confirms that `reqwest::Request` header mutation works — this is the
/// exact operation every auth strategy performs.
#[test]
fn auth_struct_reqwest_dep_request_header_mutation_int_test() {
    let url = Url::parse("https://api.example.test/").expect("valid url");
    let mut req = reqwest::Request::new(Method::POST, url);
    req.headers_mut()
        .insert("x-auth-applied", "1".parse().expect("valid header value"));
    assert_eq!(
        req.headers()
            .get("x-auth-applied")
            .and_then(|v| v.to_str().ok()),
        Some("1"),
        "header inserted into reqwest::Request must be retrievable"
    );
}

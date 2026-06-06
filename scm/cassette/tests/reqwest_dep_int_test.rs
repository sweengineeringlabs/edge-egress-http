//! Dependency coverage test for `reqwest`.
//! @covers: reqwest
//!
//! Rule 95: `reqwest` is used in `src/` and must have integration coverage
//! with an explicit `use reqwest::...` import.

use reqwest::Client;

/// @covers: reqwest
/// Verifies `reqwest::Client` is accessible and constructible, which is the
/// foundation the cassette middleware wraps.
#[test]
fn cassette_struct_dep_reqwest_client_builds_int_test() {
    let _client = Client::new();
}

/// @covers: reqwest
/// Verifies `reqwest::Method` variants are accessible — the cassette middleware
/// stores the request method as part of the match key.
#[test]
fn cassette_struct_dep_reqwest_method_variants_int_test() {
    assert_eq!(reqwest::Method::GET.as_str(), "GET");
    assert_eq!(reqwest::Method::POST.as_str(), "POST");
}

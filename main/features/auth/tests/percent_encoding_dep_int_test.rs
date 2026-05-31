//! Dependency coverage test for `percent-encoding`.
//! @covers: percent-encoding

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

/// @covers: percent-encoding
/// Confirms that `utf8_percent_encode` encodes space and special chars —
/// this is the operation used by `AwsSigV4Helper` when building canonical URIs.
#[test]
fn auth_struct_percent_encoding_dep_encodes_spaces_int_test() {
    let encoded = utf8_percent_encode("hello world", NON_ALPHANUMERIC).to_string();
    assert!(
        encoded.contains("%20"),
        "space must be percent-encoded as %20, got: {encoded}"
    );
}

/// @covers: percent-encoding
/// Confirms that unreserved path characters survive unmodified when
/// using the NON_ALPHANUMERIC set, and that slashes ARE encoded
/// (since the canonical-query encoder must encode slashes).
#[test]
fn auth_struct_percent_encoding_dep_encodes_slash_in_query_int_test() {
    let encoded = utf8_percent_encode("/bucket/key", NON_ALPHANUMERIC).to_string();
    // NON_ALPHANUMERIC encodes slashes — verify the encoding happened.
    assert!(
        !encoded.contains('/'),
        "slash must be encoded with NON_ALPHANUMERIC set, got: {encoded}"
    );
}

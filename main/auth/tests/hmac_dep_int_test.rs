//! Dependency coverage test for `hmac`.
//! @covers: hmac

#![allow(clippy::unwrap_used, clippy::expect_used)]

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// @covers: hmac
#[test]
fn test_hmac_sha256_produces_output() {
    let mut mac = HmacSha256::new_from_slice(b"key").unwrap();
    mac.update(b"data");
    let result = mac.finalize().into_bytes();
    assert_eq!(result.len(), 32, "HMAC-SHA256 must produce 32 bytes");
}

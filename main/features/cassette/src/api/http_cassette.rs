//! Primary trait for the cassette crate — re-exported from `api/traits.rs`.
//!
//! The `HttpCassette` trait is declared in `api/traits.rs`. This module
//! re-exports it for backwards-compatibility with internal use sites that
//! import via `crate::api::http_cassette::HttpCassette`.

pub use crate::api::traits::HttpCassette;

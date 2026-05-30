//! Cached response entry + associated parsing helpers for
//! RFC 7234 `Vary`, `ETag`, and RFC 5861 `stale-while-revalidate`.

mod cache_entry_helper;
mod cached_entry;
mod vary_directive;

pub(crate) use crate::api::types::CachedEntry;
pub(crate) use cache_entry_helper::CacheEntryHelper;
pub(crate) use vary_directive::VaryDirective;

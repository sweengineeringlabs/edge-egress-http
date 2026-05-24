//! Minimal usage: build the cache layer with the default config.

fn main() {
    match swe_edge_egress_cache::build_cache_layer(swe_edge_egress_cache::CacheConfig::default()) {
        Ok(_) => println!("swe_edge_egress_cache layer built"),
        Err(e) => println!("swe_edge_egress_cache: {e}"),
    }
}

//! Minimal usage: build the retry layer with the default config.

fn main() {
    match swe_edge_egress_retry::build_retry_layer(swe_edge_egress_retry::RetryConfig::default()) {
        Ok(_) => println!("swe_edge_egress_retry layer built"),
        Err(e) => println!("swe_edge_egress_retry: {e}"),
    }
}

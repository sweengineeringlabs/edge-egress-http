//! Minimal usage: build the breaker layer with the default config.

fn main() {
    match swe_edge_egress_breaker::build_breaker_layer(
        swe_edge_egress_breaker::BreakerConfig::default(),
    ) {
        Ok(_) => println!("swe_edge_egress_breaker layer built"),
        Err(e) => println!("swe_edge_egress_breaker: {e}"),
    }
}

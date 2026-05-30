//! Minimal usage: build the rate layer with the default config.

fn main() {
    match swe_edge_egress_rate::HttpRateSvc::build_rate_layer(
        swe_edge_egress_rate::RateConfig::default(),
    ) {
        Ok(_) => println!("swe_edge_egress_rate layer built"),
        Err(e) => println!("swe_edge_egress_rate: {e}"),
    }
}

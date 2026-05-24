//! Minimal usage: build the TLS layer (pass-through) and apply to a reqwest ClientBuilder.

use swe_edge_egress_tls::TlsApplier;

fn main() {
    match swe_edge_egress_tls::build_tls_layer(swe_edge_egress_tls::TlsConfig::default()) {
        Ok(layer) => match layer.apply_to(reqwest::Client::builder()) {
            Ok(_builder) => println!("swe_edge_egress_tls layer applied to ClientBuilder"),
            Err(e) => println!("swe_edge_egress_tls: apply_to failed: {e}"),
        },
        Err(e) => println!("swe_edge_egress_tls: {e}"),
    }
}

//! Minimal usage: build the loadbalancer layer with a two-backend config.

fn main() {
    use swe_edge_egress_loadbalancer::{
        BackendConfig, LoadbalancerConfig, LoadbalancerSvc, Strategy,
    };

    let config = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![
            BackendConfig {
                url: "https://api-1.internal".to_string(),
                weight: 1,
            },
            BackendConfig {
                url: "https://api-2.internal".to_string(),
                weight: 1,
            },
        ],
    };

    match LoadbalancerSvc::build_layer(config) {
        Ok(_) => println!("swe_edge_egress_loadbalancer layer built"),
        Err(e) => println!("swe_edge_egress_loadbalancer: {e}"),
    }
}

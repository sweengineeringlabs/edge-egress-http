//! Minimal usage: build a cassette layer bound to a named fixture file.

fn main() {
    match swe_edge_egress_cassette::build_cassette_layer(
        swe_edge_egress_cassette::CassetteConfig::default(),
        "example_cassette",
    ) {
        Ok(_) => println!("swe_edge_egress_cassette layer built (fixture: example_cassette.yaml)"),
        Err(e) => println!("swe_edge_egress_cassette: {e}"),
    }
}

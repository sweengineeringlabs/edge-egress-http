//! Minimal usage: build the auth middleware with the default (pass-through) config.

fn main() {
    match swe_edge_egress_auth::AuthSvc::build_auth_middleware(
        swe_edge_egress_auth::AuthConfig::default(),
    ) {
        Ok(mw) => println!("swe_edge_egress_auth middleware built: {:?}", mw),
        Err(e) => println!("swe_edge_egress_auth: {e}"),
    }
}

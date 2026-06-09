# loadbalancer

Client-side load-balancer middleware for `reqwest-middleware`. Rewrites the
request URL to a healthy backend selected by round-robin, weighted, or
least-connections strategy.

## Build

```bash
cargo build
```

## Test

```bash
cargo test
```

## Project Structure

- `main/src/api/` — Public types and traits
- `main/src/core/` — Implementations (pub(crate))
- `main/src/saf/` — Public factory facade
- `tests/` — Integration tests

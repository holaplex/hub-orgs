# Holaplex Hub Orgs

Management of Holaplex Hub organizations, projects, memberships, API credentials, and webhooks.

## Getting Started

### Dependencies

- Rust nightly-2022-12-11
- Docker for Desktop

### Commands

```
docker compose up -d
cargo run --bin migration -- -u postgres://postgres:holaplex@localhost:5437/orgs
cargo run --bin holaplex-hub-orgs
```





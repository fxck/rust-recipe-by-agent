# Rust Hello World - Local
This is a local development environment for [Rust Hello World (info + deploy)](https://app.zerops.io/recipes/rust-hello-world?environment=local) recipe on [Zerops](https://zerops.io).

<!-- #ZEROPS_EXTRACT_START:intro# -->
Local development setup — run Rust locally while connecting to a cloud PostgreSQL database. Use `zcli vpn up` to join the Zerops project network; your local `cargo run` can then reach the `db` service by hostname as if running inside the project. Deploy to the `app` container via `zcli push` to validate the production build pipeline.
<!-- #ZEROPS_EXTRACT_END:intro# -->

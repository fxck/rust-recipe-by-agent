# Rust Hello World - Local
This is the local development environment for [Rust Hello World (info + deploy)](https://app.zerops.io/recipes/rust-hello-world?environment=local) recipe on [Zerops](https://zerops.io).

<!-- #ZEROPS_EXTRACT_START:intro# -->
Cloud database with local app: a PostgreSQL service runs on Zerops while your Rust application runs on your own machine. Use `zcli vpn up` to open a secure tunnel into the Zerops network so your local `cargo run` can connect to the cloud database by hostname. Push to Zerops via `zcli push` when you want to test the production build pipeline.
<!-- #ZEROPS_EXTRACT_END:intro# -->

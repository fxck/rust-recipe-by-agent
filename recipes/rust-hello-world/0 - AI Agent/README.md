# Rust Hello World - AI Agent
This is an AI agent environment for [Rust Hello World (info + deploy)](https://app.zerops.io/recipes/rust-hello-world?environment=ai-agent) recipe on [Zerops](https://zerops.io).

<!-- #ZEROPS_EXTRACT_START:intro# -->
AI agent development environment with two Rust containers and a PostgreSQL database. The `appdev` container runs the `dev` setup — full source tree with the Cargo registry deployed — so an AI agent can SSH in and run `cargo run` immediately. The `appstage` container runs the `prod` setup for validating the release build pipeline before promoting to production.
<!-- #ZEROPS_EXTRACT_END:intro# -->

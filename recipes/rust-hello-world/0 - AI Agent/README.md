# Rust Hello World - AI Agent
This is the AI agent development environment for [Rust Hello World (info + deploy)](https://app.zerops.io/recipes/rust-hello-world?environment=ai-agent) recipe on [Zerops](https://zerops.io).

<!-- #ZEROPS_EXTRACT_START:intro# -->
Two-container workspace for AI agents: `appdev` receives the full Rust source code and pre-fetched crate registry (so `cargo run` works immediately after SSH login), while `appstage` validates the complete production build pipeline by compiling a release binary and checking PostgreSQL connectivity before serving traffic. Both containers connect to a shared PostgreSQL database.
<!-- #ZEROPS_EXTRACT_END:intro# -->

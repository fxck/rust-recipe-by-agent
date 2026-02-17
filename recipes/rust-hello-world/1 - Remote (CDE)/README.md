# Rust Hello World - Remote (CDE)
This is the remote development environment for [Rust Hello World (info + deploy)](https://app.zerops.io/recipes/rust-hello-world?environment=remote-cde) recipe on [Zerops](https://zerops.io).

<!-- #ZEROPS_EXTRACT_START:intro# -->
Two-container workspace for remote development via SSH or IDE mounting (VS Code Remote-SSH, JetBrains Gateway): `appdev` holds the full Rust source and pre-fetched crate registry ready for `cargo run`, while `appstage` runs the production build pipeline so you can validate release builds before promoting to production. Both containers share a PostgreSQL database.
<!-- #ZEROPS_EXTRACT_END:intro# -->

# Rust Hello World Recipe

<!-- #ZEROPS_EXTRACT_START:intro# -->
Actix Web application with a health check endpoint that verifies PostgreSQL connectivity using `tokio-postgres` and returns JSON service status. Demonstrates the full Zerops build pipeline for Rust: compiling a release binary with `cargo build --release --locked`, deploying the minimal artifact to an Ubuntu runtime, and using `CARGO_HOME` to keep the Cargo registry inside the project directory for efficient caching.
<!-- #ZEROPS_EXTRACT_END:intro# -->

**Full recipe page and deploy with one-click**

[![Deploy on Zerops](https://github.com/zeropsio/recipe-shared-assets/blob/main/deploy-button/light/deploy-button.svg)](https://app.zerops.io/recipes/rust-hello-world?environment=small-production)

![rust](https://github.com/zeropsio/recipe-shared-assets/blob/main/covers/svg/cover-rust.svg)

Offered in examples for the whole development lifecycle - from environments for AI agents like [Claude Code](https://www.anthropic.com/claude-code) or [opencode](https://opencode.ai) through environments for remote (CDE) or local development of each developer to stage and productions of all sizes.

- **AI agent** [[info]](/0%20-%20AI%20Agent) - [[deploy with one click]](https://app.zerops.io/recipes/rust-hello-world?environment=ai-agent)
- **Remote (CDE)** [[info]](/1%20-%20Remote%20(CDE)) - [[deploy with one click]](https://app.zerops.io/recipes/rust-hello-world?environment=remote-cde)
- **Local** [[info]](/2%20-%20Local) - [[deploy with one click]](https://app.zerops.io/recipes/rust-hello-world?environment=local)
- **Stage** [[info]](/3%20-%20Stage) - [[deploy with one click]](https://app.zerops.io/recipes/rust-hello-world?environment=stage)
- **Small Production** [[info]](/4%20-%20Small%20Production) - [[deploy with one click]](https://app.zerops.io/recipes/rust-hello-world?environment=small-production)
- **Highly-available Production** [[info]](/5%20-%20Highly-available%20Production) - [[deploy with one click]](https://app.zerops.io/recipes/rust-hello-world?environment=highly-available-production)

---

For more advanced examples see all [Rust recipes](https://app.zerops.io/recipes?lf=rust) on Zerops.

Need help setting your project up? Join [Zerops Discord community](https://discord.gg/zeropsio).

# Rust Hello World Recipe App
Actix-web application with a health check endpoint at `/` that verifies live PostgreSQL connectivity — returning `{"type":"rust","status":{"database":"OK"}}` on success or HTTP 503 with an error detail if the database is unreachable.
Used within [Rust Hello World recipe](https://app.zerops.io/recipes/rust-hello-world) for [Zerops](https://zerops.io) platform.

**Full recipe page and deploy with one-click**

[![Deploy on Zerops](https://github.com/zeropsio/recipe-shared-assets/blob/main/deploy-button/light/deploy-button.svg)](https://app.zerops.io/recipes/rust-hello-world?environment=small-production)

![Rust cover](https://github.com/zeropsio/recipe-shared-assets/blob/main/covers/svg/cover-rust.svg)

## Integration Guide

<!-- #ZEROPS_EXTRACT_START:integration-guide# -->

### 1. Adding `zerops.yaml`
The main application configuration file you place at the root of your repository, it tells Zerops how to build, deploy and run your application.

```yaml
zerops:
  # Production setup: Compile a release binary and deploy only that artifact.
  # The binary is self-contained — no runtime dependencies beyond the OS libc —
  # so the deployed footprint is a single file.
  # Contrast with the 'dev' setup below, which deploys full source code so a
  # developer can edit and run the app directly via SSH.
  - setup: prod
    build:
      base: rust@stable

      # build.envVariables are injected into every build command in this
      # setup — prepareCommands, buildCommands, and cache steps all see them.
      # CARGO_HOME is redirected into the project directory (./.cargo) so the
      # cache section below can persist the downloaded crate registry between
      # builds. The default location (~/.cargo) is outside the project tree
      # and therefore outside the reach of Zerops build caching.
      envVariables:
        CARGO_HOME: ./.cargo

      buildCommands:
        # --release: Enable full compiler optimisations (LTO, inlining) for
        # a production binary. Compile time is higher but runtime performance
        # and binary size are optimal.
        # --locked: Require that Cargo.lock matches Cargo.toml exactly.
        # Cargo.lock is committed to version control, so every build resolves
        # to the same dependency versions — no silent upgrades between deploys.
        - cargo build --release --locked

      deployFiles:
        # Deploy only the compiled binary. Path must match the start command
        # exactly: the binary lands at /var/www/target/release/rust-hello-world
        # in the runtime container.
        - ./target/release/rust-hello-world

      # Cache the crate registry so subsequent builds skip re-downloading
      # crates from crates.io. Must match the CARGO_HOME path set above.
      cache:
        - .cargo/registry

    # Readiness check: Zerops calls GET / on the new runtime container
    # during deployment. The project balancer withholds traffic until this
    # returns HTTP 200, giving the app time to connect to PostgreSQL and
    # confirm it is healthy before receiving real requests.
    # Docs: https://docs.zerops.io/zerops-yaml/specification#readinesscheck-
    deploy:
      readinessCheck:
        httpGet:
          port: 3000
          path: /

    run:
      base: rust@stable
      ports:
        - port: 3000
          httpSupport: true
      envVariables:
        # DB_* variables reference the 'db' service using the
        # {hostname}_{key} pattern. When the import.yaml hostname is 'db',
        # Zerops injects db_hostname, db_port, db_user, db_password, etc.
        # Docs: https://docs.zerops.io/features/env-variables#referencing-variables
        DB_NAME: db
        DB_HOST: ${db_hostname}
        DB_PORT: ${db_port}
        DB_USER: ${db_user}
        DB_PASS: ${db_password}
      start: ./target/release/rust-hello-world

  # Development setup: Deploy full source code and a pre-fetched dependency
  # cache so a developer can SSH in and run 'cargo run' immediately.
  # The build container downloads crates; the runtime container holds the
  # working directory ready for interactive development.
  - setup: dev
    build:
      base: rust@stable

      # Redirect CARGO_HOME into the project tree for Zerops build caching.
      # Pattern explained at 'prod' setup above.
      envVariables:
        CARGO_HOME: ./.cargo

      buildCommands:
        # Download all crates declared in Cargo.toml into the local registry.
        # 'cargo fetch' does not compile — that is left to the developer via SSH.
        # Using 'cargo fetch' (not 'cargo build') avoids a full compilation
        # that would be discarded the moment the developer edits source code.
        - cargo fetch

      # Deploy the entire working directory: source code + downloaded crates.
      # The developer gets a fully prepared workspace via SSH.
      deployFiles: ./

      # Cache the crate registry between dev deploys so 'cargo fetch' only
      # downloads new or updated crates.
      cache:
        - .cargo/registry

    run:
      # rust@stable provides the full Rust toolchain (cargo, rustc) for
      # interactive development. The Zerops rust@stable image is Ubuntu-based
      # (add 'os: alpine' in zerops.yaml if you want an Alpine runtime).
      # This gives developers cargo run support via SSH immediately.
      base: rust@stable
      ports:
        - port: 3000
          httpSupport: true
      envVariables:
        DB_NAME: db
        DB_HOST: ${db_hostname}
        DB_PORT: ${db_port}
        DB_USER: ${db_user}
        DB_PASS: ${db_password}
      # zsc noop keeps the container alive without starting the app.
      # The developer starts the server manually via SSH: 'cargo run'
      # or 'CARGO_HOME=./.cargo cargo run'.
      start: zsc noop --silent
```

<!-- #ZEROPS_EXTRACT_END:integration-guide# -->

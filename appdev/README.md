# Rust Hello World Recipe App
Simple Rust API with single endpoint that reads from and writes to a PostgreSQL database. Used within [Rust Hello World recipe](https://app.zerops.io/recipes/rust-hello-world) for [Zerops](https://zerops.io) platform.

**Full recipe page and deploy with one-click**

[![Deploy on Zerops](https://github.com/zeropsio/recipe-shared-assets/blob/main/deploy-button/light/deploy-button.svg)](https://app.zerops.io/recipes/rust-hello-world?environment=small-production)

![Rust cover](https://github.com/zeropsio/recipe-shared-assets/blob/main/covers/svg/cover-rust.svg)

## Integration Guide

<!-- #ZEROPS_EXTRACT_START:integration-guide# -->

### 1. Adding `zerops.yaml`
The main application configuration file you place at the root of your repository, it tells Zerops how to build, deploy and run your application.

```yaml
zerops:
  # Production setup: Build optimized binary for staging/production deployment.
  # Compiles Rust in release mode (optimizations enabled, debug symbols stripped),
  # deploys only the compiled binary. Used by 'appstage' (Env 0-3) and 'app'
  # (Env 4-5) services. In contrast, 'dev' setup deploys full source for live development.
  - setup: prod
    build:
      # Rust stable provides latest stable compiler with backwards compatibility guarantees.
      # Match build and runtime versions to ensure binary compatibility — Rust binaries
      # are statically linked but depend on glibc version matching between environments.
      base: rust@stable

      buildCommands:
        # Build release binary with full optimizations (--release flag).
        # Release mode enables level 3 optimizations, strips debug symbols, and produces
        # ~10x smaller binaries than debug builds. For example, a simple Actix app goes
        # from 50MB (debug) to 5MB (release). Build time increases but runtime performance
        # improves significantly (loop unrolling, inlining, dead code elimination).
        - cargo build --release --locked

      # Deploy only the compiled binary — Rust produces standalone executables.
      # Pattern: ./target/release/{package_name} where package_name comes from Cargo.toml.
      # Notice we use the exact binary name, not a glob pattern (globs often fail in Zerops).
      deployFiles:
        - ./target/release/rust-hello-world

      # Cache Cargo registry and git dependencies to speed up subsequent builds.
      # First build downloads all crates (~200MB for Actix stack), subsequent builds
      # reuse cached dependencies and only rebuild changed code (30s vs 5min builds).
      cache:
        - ~/.cargo/registry
        - ~/.cargo/git

    # Readiness check verifies containers are ready DURING DEPLOYMENT before
    # the project balancer routes traffic to them. Prevents "connection refused"
    # errors during zero-downtime deployments.
    # Docs: https://docs.zerops.io/zerops-yaml/specification#readinesscheck-
    deploy:
      readinessCheck:
        httpGet:
          port: 3000
          path: /

    run:
      # Match build environment (rust@stable) to ensure glibc compatibility.
      # Rust binaries depend on system libraries — version mismatches cause
      # "GLIBC_X.XX not found" errors at runtime.
      base: rust@stable

      ports:
        - port: 3000
          httpSupport: true

      # Database connection using Zerops environment variable pattern.
      # Pattern: {hostname}_{credential} — since we name the service 'db',
      # Zerops provides db_hostname, db_port, db_user, db_password automatically.
      # Docs: https://docs.zerops.io/features/env-variables#referencing-variables
      envVariables:
        DB_NAME: db
        DB_HOST: ${db_hostname}
        DB_PORT: ${db_port}
        DB_USER: ${db_user}
        DB_PASS: ${db_password}

      # Run the compiled binary directly (no cargo needed in runtime).
      # Path matches deployFiles structure — binary is deployed at target/release/.
      # Rust executables are self-contained and don't require the Rust toolchain at runtime.
      start: ./target/release/rust-hello-world

  # Development setup: Deploy full source code for live development via SSH.
  # AI agents and developers SSH into 'appdev' containers to build, test, and iterate.
  # Uses ubuntu for richer toolset (debugging tools, editors) vs minimal alpine.
  - setup: dev
    build:
      base: rust@stable

      # Pre-download dependencies on build container so they're ready when developer SSHs in.
      # Using 'cargo fetch' (not 'cargo build') — downloads dependencies without compiling,
      # saving build time. Developer compiles manually after SSH for faster iteration.
      buildCommands:
        - cargo fetch

      # Deploy entire source tree for development — developers modify code and rebuild.
      # In contrast to prod which deploys only the binary, dev needs Cargo.toml, src/, etc.
      deployFiles:
        - ./

      cache:
        - ~/.cargo/registry
        - ~/.cargo/git

    run:
      # Ubuntu provides familiar development environment with apt, curl, vim, etc.
      # Rust installation adds ~1GB but includes full toolchain (cargo, rustc, rustfmt).
      os: ubuntu
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

      # Pre-fetch dependencies each time container starts (before developer SSHs in).
      # Since we deployed full source, dependencies need to be available immediately.
      # Pattern: Run cargo fetch to download crates without compiling.
      initCommands:
        - cargo fetch

      # No automatic process — developer manually runs 'cargo run' after SSH.
      # This gives full control over development workflow (build, test, debug, restart).
      start: zsc noop --silent
```

<!-- #ZEROPS_EXTRACT_END:integration-guide# -->

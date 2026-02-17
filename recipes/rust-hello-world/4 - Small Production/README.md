# Rust Hello World - Small Production
This is the small production environment for [Rust Hello World (info + deploy)](https://app.zerops.io/recipes/rust-hello-world?environment=small-production) recipe on [Zerops](https://zerops.io).

<!-- #ZEROPS_EXTRACT_START:intro# -->
Production-grade deployment with two always-on Rust containers for zero-downtime deploys and a non-HA PostgreSQL database. The Actix-web server compiles to a compact release binary (~6 MB) with minimal RAM requirements — autoscaling handles traffic spikes. The health check at `/` verifies live PostgreSQL connectivity before each new container receives traffic from the project balancer.
<!-- #ZEROPS_EXTRACT_END:intro# -->

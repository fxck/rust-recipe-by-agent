# Rust Hello World - Small Production
This is a small production environment for [Rust Hello World (info + deploy)](https://app.zerops.io/recipes/rust-hello-world?environment=small-production) recipe on [Zerops](https://zerops.io).

<!-- #ZEROPS_EXTRACT_START:intro# -->
Production-ready environment for moderate traffic. Runs a minimum of 2 containers for zero-downtime deployments and load balancer failover. Autoscaling adjusts resources based on actual load, with reserved RAM headroom for the Tokio async runtime during traffic spikes.
<!-- #ZEROPS_EXTRACT_END:intro# -->

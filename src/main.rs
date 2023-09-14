#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // -----------------------------------------------------------------------------
    // Development-only
    // NOTE: this is only required for early development.
    // NOTE: no `?` operator on the `await`; this should simply explode if there's an issue.
    // FIXME: place this behind a config flag once initial development is complete.
    orrery::initialise_development_environment().await;
    // -----------------------------------------------------------------------------

    println!("Hello, World!");
}

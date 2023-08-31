#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Grab dem config values.
    let config = orrery::parse_app_config();
    // Initialise the tracing subscriber.
    orrery::init_tracing();
    // Set up the database connection pool.
    let db_pool = orrery::construct_db_pool(&config.database_url).await?;
    // Run any outstanding migrations.
    // TODO: move this to a build.rs file
    orrery::run_db_migrations(&db_pool).await?;

    orrery::serve(config, db_pool).await?;

    Ok(())
}

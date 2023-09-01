#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Grab dem config values.
    let config = orrery::parse_app_config();
    // Initialise the tracing subscriber.
    orrery::init_tracing();
    // Set up the database connection pool.
    let db_conn = orrery::DatabaseConnection::new(&config.database_url).await?;
    // Start the server.
    orrery::serve(config, db_conn).await?;

    Ok(())
}

// -------------------------------------------------------------------------------
// 1. Collected imports & those exposed as the lib API
// -------------------------------------------------------------------------------

/// Defines the arguments required to start the server application using [`clap`]
//
/// [`clap`]: https://github.com/clap-rs/clap/
mod config;

/// Custom error handling: this is placed at root because it wraps errors related to both
/// the API and other parts of the application. Axum provides a trait called `IntoResponse`,
/// which allows you to return `Result` from handler functions: the error type builds on this.
///
/// [`axum::response::IntoResponse`]: https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html
/// [`anyhow`]: https://github.com/dtolnay/anyhow
/// [`thiserror`]: https://github.com/dtolnay/thiserror
mod error;

mod routes;

mod data;

/// Public interface for `docket`
pub use {
    config::{init_tracing, parse_app_config, AppConfig},
    error::{AppError, AppResult},
};

// -------------------------------------------------------------------------------
// 2. Application state
// -------------------------------------------------------------------------------

/// Common application state
/// Defines the `AppContext` struct, representing gloabl state, which allows handlers to
/// access common application state.
#[derive(Clone, axum::extract::FromRef)]
pub struct AppContext {
    /// The application config struct, defined in `src/config.rs`
    config: AppConfig,
    /// The database connection pool
    db_pool: sqlx::sqlite::SqlitePool,
}

// -------------------------------------------------------------------------------
// 3. Modularised functions used directly in the app `main` function
// -------------------------------------------------------------------------------
use anyhow::Context;

/// Construction of the Db pool is separated to make it easier to inject an in-memory verison into tests.
/// For tests, the `DATABASE_URL` env var can be set to `:memory:`, which will cause the in-memory version to be used.
pub async fn construct_db_pool(
    database_url: &str,
) -> AppResult<sqlx::sqlite::SqlitePool, anyhow::Error> {
    sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .context("Failed to connect to database")
}

pub async fn run_db_migrations(db_pool: &sqlx::sqlite::SqlitePool) -> anyhow::Result<()> {
    sqlx::migrate!().run(db_pool).await?;

    Ok(())
}

pub async fn serve(config: AppConfig, db_pool: sqlx::sqlite::SqlitePool) -> anyhow::Result<()> {
    use std::net::{IpAddr, Ipv6Addr, SocketAddr};

    // REVIEW: is it fine to just use IPV6 here, or should there be an IPV4 fallback?
    let socket_address = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), config.server_port);

    // Merge the API and static routes, attach the tracing layer, and make the application state available.
    let router = routes::construct_routes(AppContext { config, db_pool });

    tracing::debug!("server listening on {:?}", socket_address);

    axum::Server::bind(&socket_address)
        .serve(router.into_make_service())
        .await
        .with_context(|| "failed to start server")?;

    Ok(())
}

#[cfg(test)]
///! # Testing
///!
///! The core setup for tests is purposefully colocated with the main library code, as it
///! directly uses the setup functions provided at the library root.
///!
///! TODO: move this to either a `main_test.rs` or a `lib_test.rs` file. I feel it is important
///! TODO: to keep the test setup colocated with the main library code, but it's also important
///! TODO: to keep the test setup separate from the main application code. That seems to strike
///! TODO: a reasonable balance.
pub mod core_test_setup {
    use super::*;
    use axum_test_helper::TestClient;

    /// Construct a test client, with a database connection pool pointing to an in-memory database
    /// and a set of routes.
    /// TODO: pass the `Router` and any database prep functions in as arguments.
    /// TODO: MUST CHECK THAT THE DATABASE IS DESTROYED ON PULLDOWN.
    pub async fn construct_test_client() -> anyhow::Result<TestClient> {
        let config = parse_app_config();
        let db_pool = construct_db_pool(":memory:").await?;
        let router = routes::construct_routes(AppContext { config, db_pool });

        Ok(TestClient::new(router))
    }
}

mod tests {
    #[tokio::test]
    async fn smoke_test() {
        let client = crate::core_test_setup::construct_test_client()
            .await
            .unwrap();

        let response = client.get("/").send().await;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(response.text().await, "Hello, World!");
    }
}

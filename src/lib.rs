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

/// Application state, which is passed to handlers and contains shared resources.
mod state;

/// Database connection pool, which is passed to handlers via the application state, along
/// with the handlers themselves.
mod datalayer;

/// Public interface for `orrery`
pub use {
    config::{init_tracing, parse_app_config, AppConfig},
    datalayer::DatabaseConnection,
    error::{AppError, AppResult},
    state::AppState,
};

// -------------------------------------------------------------------------------
// 2. Modularised functions used directly in the app `main` function
// -------------------------------------------------------------------------------
use anyhow::Context;
use axum::Router;

/// Construction of the Db pool is separated to make it easier to inject an in-memory verison into tests.
/// For tests, the `DATABASE_URL` env var can be set to `:memory:`, which will cause the in-memory version to be used.

pub async fn serve(
    config: AppConfig,
    db_conn: datalayer::DatabaseConnection,
) -> anyhow::Result<()> {
    use std::net::{IpAddr, Ipv6Addr, SocketAddr};
    // REVIEW: is it fine to just use IPV6 here, or should there be an IPV4 fallback?
    let socket_address = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), config.server_port);

    // Merge the API and static routes, attach the tracing layer, and make the application state available.
    let router = Router::new()
        .merge(datalayer::api_routes())
        .with_state(AppState { config, db_conn });

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
    use crate::datalayer::DatabaseConnection;

    use super::*;
    use axum::Router;
    use axum_test_helper::TestClient;

    /// Construct a test client, with a database connection pool pointing to an in-memory database
    /// and a set of routes.
    /// TODO: pass the `Router` and any database prep functions in as arguments.
    /// TODO: MUST CHECK THAT THE DATABASE IS DESTROYED ON PULLDOWN.
    pub async fn construct_test_client(
        router: Router<AppState>,
    ) -> AppResult<TestClient, anyhow::Error> {
        let config = parse_app_config();
        let db_conn = DatabaseConnection::new(":memory:").await?;
        let state = AppState { config, db_conn };

        Ok(TestClient::new(router.with_state(state)))
    }
}

mod tests {
    #[tokio::test]
    async fn smoke_test() {
        let router =
            axum::Router::new().route("/", axum::routing::get(|| async { "Hello, World!" }));

        let client = crate::core_test_setup::construct_test_client(router)
            .await
            .unwrap();

        let response = client.get("/").send().await;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(response.text().await, "Hello, World!");
    }
}

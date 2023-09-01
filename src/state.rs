use crate::{datalayer::DatabaseConnection, AppConfig};

/// Common application state, allows handlers to access shared resources.
#[derive(Clone, axum::extract::FromRef)]
pub struct AppState {
    /// The application config struct, defined in `src/config.rs`
    pub config: AppConfig,

    /// The database connection pool
    pub db_conn: DatabaseConnection,
}

//! Utilities for local development.
//!
//! TODO: document
use sqlx::sqlite::{Sqlite, SqliteConnectOptions, SqlitePoolOptions};
use sqlx::Pool;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::OnceCell;

use crate::data_access::DataAccessManager;

type Db = Pool<Sqlite>;

// Hardcoded config values to ensure no overlap with the real config.
const DATABASE_URL: &str = ":memory:";
// Hardcoded path to the SQL files used for testing.
const SQL_DIRECTORY: &str = "migrations";

async fn create_development_database_pool(db_connection_url: &str) -> Result<Db, sqlx::Error> {
    let connection_options = SqliteConnectOptions::new()
        .filename(db_connection_url)
        .create_if_missing(true);

    let connection_pool = SqlitePoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_millis(500))
        .connect_with(connection_options)
        .await;

    connection_pool
}

/// Manually execute the SQL files in the `sql/dev` folder.
/// This is fiddly, but absolutely ensures control over the initialisation of the database.
async fn execute_sql_statements_from_file(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    let sql_file_content = fs::read_to_string(file)?;
    let sql_statements: Vec<&str> = sql_file_content.split(';').collect();

    for statement in sql_statements {
        sqlx::query(statement).execute(db).await?;
    }

    Ok(())
}

/// Initialise the test database
async fn initialise_development_database() -> Result<Db, Box<dyn std::error::Error>> {
    let mut sql_filepaths: Vec<PathBuf> = fs::read_dir(SQL_DIRECTORY)?
        .filter_map(|path_buf| path_buf.ok().map(|pbuf| pbuf.path()))
        .collect();
    // NOTE: the SQL files are prefixed sequentially - 00_, 01_, 02_, etc.
    //       So the `sort` method will ensure they are executed in the correct order.
    sql_filepaths.sort();

    let db_pool = create_development_database_pool(DATABASE_URL).await?;

    for sql_filepath in sql_filepaths {
        if let Some(sql_filepath) = sql_filepath.to_str() {
            // NOTE: fix for Windows paths.
            let sql_filepath = sql_filepath.replace("\\", "/");
            execute_sql_statements_from_file(&db_pool, &sql_filepath).await?;
        }
    }

    Ok(db_pool)
}

/// Initialise environment from local development.
pub async fn initialise_development_environment() {
    // NOTE: Tokio's `OnceCell` is used rather than `OnceLock` from Rust's stdlib.
    //       `OnceCell` is designed for async contexts.
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        // NOTE: `unwrap` used here as want eveything to explode as soon as possible if there's an issue;
        //       this would be an unrecoverable error, so no point in trying to handle it.
        let _ = initialise_development_database().await.unwrap();
    })
    .await;
}

#[cfg(test)]
pub async fn initialise_test_environment() -> DataAccessManager {
    // NOTE: Exact same setup pattern as the development environment setup, using the OnceCell,
    //       but this time the `DataAccessManager` is returned.
    static INIT: OnceCell<DataAccessManager> = OnceCell::const_new();

    let dam = INIT
        .get_or_init(|| async {
            let db_pool = initialise_development_database().await.unwrap();
            // NOTE: unwrap again here, fail fast and fail early for tests.
            DataAccessManager::new_from_existing_resources(db_pool)
                .await
                .unwrap()
        })
        .await;

    // The data access manager is *designed* to be cloned, so following is fine.
    dam.clone()
}

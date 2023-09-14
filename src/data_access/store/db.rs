use crate::config::config;
use crate::data_access::{Error, Result};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};
use std::time::Duration;

// -----------------------------------------------------------------------------
// Sqlite database setup/connection handling
// -----------------------------------------------------------------------------

pub type DbPool = Pool<Sqlite>;

pub async fn create_database_pool() -> Result<DbPool> {
    let connection_pool = SqlitePoolOptions::new()
        .max_connections(config().DATABASE_POOL_MAX_CONNECTIONS)
        .acquire_timeout(Duration::from_millis(
            config().DATABASE_POOL_CONNECTION_TIMEOUT_MS,
        ))
        .connect(&config().DATABASE_URL)
        .await
        .map_err(|err| Error::FailedToCreatePool(err.to_string()));

    connection_pool
}

// -----------------------------------------------------------------------------
// Generic CRUD controllers
//
// TODO: implement. The issue here is that SQLB, the library used to build the generic
//       SQL queries, does not support SQLite. This is not an issue for reading or deleting,
//       but for creating/updating, need a way to abstract over field names and values.
//       SQLB provides a way to do this easily, in turn allowing for generic controller definition.
//       Without this ability, the controllers will need to be defined for each entity, thus
//       defeating the purpose of the generic controllers.
// -----------------------------------------------------------------------------

use crate::config;
use crate::data_access::{DataAccessManager, Error, Result};
use crate::RequestContext;
use sqlx::sqlite::{SqlitePoolOptions, SqliteRow};
use sqlx::{FromRow, Pool, Sqlite};
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

pub trait DbCrudServer {
    const TABLE: &'static str;
}

pub struct DbCrudAction;

impl DbCrudAction {
    pub async fn create<DBCS, E>(
        _ctx: &RequestContext,
        _dam: &DataAccessManager,
        _data: E,
    ) -> Result<i64>
    where
        DBCS: DbCrudServer,
        E: for<'r> FromRow<'r, SqliteRow> + Unpin + Send,
    {
        unimplemented!()
    }

    pub async fn read<DBCS, E>(_ctx: &RequestContext, dam: &DataAccessManager, id: i64) -> Result<E>
    where
        DBCS: DbCrudServer,
        E: for<'r> FromRow<'r, SqliteRow> + Unpin + Send,
    {
        let db = dam.db_pool();
        let sql = format!("SELECT * FROM {} WHERE id = $1", DBCS::TABLE);

        let entity: E = sqlx::query_as(&sql)
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: DBCS::TABLE,
                id,
            })?;

        Ok(entity)
    }

    pub async fn read_all<DBCS, E>(_ctx: &RequestContext, dam: &DataAccessManager) -> Result<Vec<E>>
    where
        DBCS: DbCrudServer,
        E: for<'r> FromRow<'r, SqliteRow> + Unpin + Send,
    {
        let db = dam.db_pool();
        let sql = format!("SELECT * FROM {}", DBCS::TABLE);

        let entities: Vec<E> = sqlx::query_as(&sql).fetch_all(db).await?;

        Ok(entities)
    }

    pub async fn update<DBCS, E>(
        _ctx: &RequestContext,
        _dam: &DataAccessManager,
        _data: E,
    ) -> Result<i64>
    where
        DBCS: DbCrudServer,
        E: for<'r> FromRow<'r, SqliteRow> + Unpin + Send,
    {
        unimplemented!()
    }

    pub async fn delete<DBCS, E>(
        _ctx: &RequestContext,
        dam: &DataAccessManager,
        id: i64,
    ) -> Result<()>
    where
        DBCS: DbCrudServer,
        E: for<'r> FromRow<'r, SqliteRow> + Unpin + Send,
    {
        let db = dam.db_pool();
        let sql = format!("DELETE FROM {} WHERE id = $1", DBCS::TABLE);

        let delete_count: u64 = sqlx::query(&sql)
            .bind(id)
            .execute(db)
            .await?
            .rows_affected();

        if delete_count == 0 {
            Err(Error::EntityNotFound {
                entity: DBCS::TABLE,
                id,
            })
        } else {
            Ok(())
        }
    }
}

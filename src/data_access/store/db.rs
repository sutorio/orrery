use crate::config::get_config;
use crate::data_access::{DataAccessManager, Error, Result};
use crate::RequestContext;
use sqlb::HasFields;
use sqlx::sqlite::{SqlitePoolOptions, SqliteRow};
use sqlx::{FromRow, Pool, Sqlite};
use std::time::Duration;

// -----------------------------------------------------------------------------
// Sqlite database setup/connection handling
// -----------------------------------------------------------------------------

pub type DbPool = Pool<Sqlite>;

pub async fn create_database_pool() -> Result<DbPool> {
    let connection_pool = SqlitePoolOptions::new()
        .max_connections(get_config().DATABASE_POOL_MAX_CONNECTIONS)
        .acquire_timeout(Duration::from_millis(
            get_config().DATABASE_POOL_CONNECTION_TIMEOUT_MS,
        ))
        .connect(&get_config().DATABASE_URL)
        .await
        .map_err(|err| Error::FailedToCreatePool(err.to_string()));

    connection_pool
}

// -----------------------------------------------------------------------------
// Generic CRUD controllers
// -----------------------------------------------------------------------------

pub trait DbCrudServer {
    const TABLE: &'static str;
}

pub struct DbCrudAction;

impl DbCrudAction {
    pub async fn create<DBCS, E>(
        _ctx: &RequestContext,
        dam: &DataAccessManager,
        data: E,
    ) -> Result<i64>
    where
        DBCS: DbCrudServer,
        E: HasFields,
    {
        let (id,) = sqlb::insert()
            .table(DBCS::TABLE)
            .data(data.not_none_fields())
            .returning(&["id"])
            .fetch_one::<_, (i64,)>(dam.db_pool())
            .await?;

        Ok(id)
    }

    pub async fn read<DBCS, E>(_ctx: &RequestContext, dam: &DataAccessManager, id: i64) -> Result<E>
    where
        DBCS: DbCrudServer,
        E: for<'r> FromRow<'r, SqliteRow> + Unpin + Send,
        E: HasFields,
    {
        let entity: E = sqlb::select()
            .table(DBCS::TABLE)
            .columns(E::field_names())
            .and_where("id", "=", id)
            .fetch_optional(dam.db_pool())
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
        E: HasFields,
    {
        let entities: Vec<E> = sqlb::select()
            .table(DBCS::TABLE)
            .columns(E::field_names())
            .order_by("id")
            .fetch_all(dam.db_pool())
            .await?;

        Ok(entities)
    }

    pub async fn update<DBCS, E>(
        _ctx: &RequestContext,
        dam: &DataAccessManager,
        id: i64,
        data: E,
    ) -> Result<()>
    where
        DBCS: DbCrudServer,
        E: HasFields,
    {
        let update_count = sqlb::update()
            .table(DBCS::TABLE)
            .and_where("id", "=", id)
            .data(data.not_none_fields())
            .exec(dam.db_pool())
            .await?;

        if update_count == 0 {
            Err(Error::EntityNotFound {
                entity: DBCS::TABLE,
                id,
            })
        } else {
            Ok(())
        }
    }

    pub async fn delete<DBCS>(_ctx: &RequestContext, dam: &DataAccessManager, id: i64) -> Result<()>
    where
        DBCS: DbCrudServer,
    {
        let delete_count = sqlb::delete()
            .table(DBCS::TABLE)
            .and_where("id", "=", id)
            .exec(dam.db_pool())
            .await?;

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

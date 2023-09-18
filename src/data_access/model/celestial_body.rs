use crate::data_access::{DataAccessManager, DbCrudAction, DbCrudServer, Error, Result};
use crate::RequestContext;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// -----------------------------------------------------------------------------
// Types
// -----------------------------------------------------------------------------

const TABLE_NAME: &'static str = "celestial_body";

/// Returned from the data access layer, hence `Serialize`.
/// This is the "entity" that is used by the application. It maps to database table/s,
/// so sqlx's `FromRow` is implemented.
#[derive(Clone, Debug, FromRow, Serialize)]
pub struct CelestialBody {
    pub body_id: i64,
    pub body_name: String,
}

/// Sent to the data access layer, hence `Deserialize`.
#[derive(Deserialize)]
pub struct CelestialBodyCreate {
    pub body_name: String,
}

/// Sent to the data access layer, hence `Deserialize`.
#[derive(Deserialize)]
pub struct CelestialBodyUpdate {
    pub body_name: Option<String>,
}

// -----------------------------------------------------------------------------
// Server
//
// TODO: this should be generalised into a trait + generic functions, and then
//       implemented for each entity. This *should* allow the `Client` to just
//       call the generic functions. However, this is currently difficult, see
//       the notes in `src/data_access/mod.rs`.
// -----------------------------------------------------------------------------

struct Server;

impl DbCrudServer for Server {
    const TABLE: &'static str = "celestial_body";
}

impl Server {
    async fn create(
        _ctx: &RequestContext,
        dac: &DataAccessManager,
        data: CelestialBodyCreate,
    ) -> Result<i64> {
        let db = dac.db_pool();
        let result = sqlx::query!(
            r#"
                INSERT INTO celestial_body (body_name)
                VALUES ($1)
            "#,
            data.body_name
        )
        .execute(db)
        .await?
        .last_insert_rowid();

        Ok(result)
    }

    async fn read(
        _ctx: &RequestContext,
        dam: &DataAccessManager,
        id: i64,
    ) -> Result<CelestialBody> {
        DbCrudAction::read::<Self, _>(_ctx, dam, id).await
    }

    async fn read_all(
        _ctx: &RequestContext,
        dam: &DataAccessManager,
    ) -> Result<Vec<CelestialBody>> {
        DbCrudAction::read_all::<Self, _>(_ctx, dam).await
    }

    async fn update(
        _ctx: &RequestContext,
        dam: &DataAccessManager,
        _id: i64,
        _data: CelestialBodyUpdate,
    ) -> Result<i64> {
        unimplemented!()
    }

    async fn delete(_ctx: &RequestContext, dam: &DataAccessManager, id: i64) -> Result<()> {
        DbCrudAction::delete::<Self, CelestialBody>(_ctx, dam, id).await
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RequestContext, _dev_utils::initialise_test_environment};
    use anyhow::Result;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // Setup
        let dam = initialise_test_environment().await;
        let ctx = RequestContext::root_context();

        // Fixtures
        let fixture_name = "test_create_ok";

        // Execution¬
        let data = CelestialBodyCreate {
            body_name: fixture_name.to_string(),
        };
        let result_id = Server::create(&ctx, &dam, data).await?;

        // Verification
        let (body_name,): (String,) = sqlx::query_as(
            "
                SELECT body_name
                FROM celestial_body
                WHERE body_id = $1
            ",
        )
        .bind(result_id)
        .fetch_one(dam.db_pool())
        .await?;

        assert_eq!(body_name, fixture_name);

        // Cleanup
        let delete_count = sqlx::query("DELETE FROM celestial_body WHERE body_id = $1")
            .bind(result_id)
            .execute(dam.db_pool())
            .await?
            .rows_affected();

        assert_eq!(delete_count, 1, "Expected 1 row to be deleted");

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_not_found() -> Result<()> {
        // Setup
        let dam = initialise_test_environment().await;
        let ctx = RequestContext::root_context();

        // Execution¬
        let result = Server::read(&ctx, &dam, 0).await;

        assert!(matches!(
            result,
            Err(Error::EntityNotFound {
                entity: TABLE_NAME,
                id: 0
            })
        ));

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_not_found() -> Result<()> {
        // Setup
        let dam = initialise_test_environment().await;
        let ctx = RequestContext::root_context();

        // Execution¬
        let result = Server::delete(&ctx, &dam, 0).await;

        assert!(matches!(
            result,
            Err(Error::EntityNotFound {
                entity: TABLE_NAME,
                id: 0
            })
        ));

        Ok(())
    }
}

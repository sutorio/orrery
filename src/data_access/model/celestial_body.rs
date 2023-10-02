use crate::data_access::store::db::{DbCrudAction, DbCrudServer};
use crate::data_access::{DataAccessManager, Result};
use crate::RequestContext;
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;

// -----------------------------------------------------------------------------
// Types
// -----------------------------------------------------------------------------

// SQLite does not support enums. This is not a huge issue: a table can be created
// to store the enum values, and then a foreign key can be used to reference them.
// However, to save on boilerplate, this will be enforced outside of the database.
// REVIEW: table + foreign keys is sensible, but is it worth the effort?

pub enum CelestialRegion {
    InnerSolarSystem,
    OuterSolarSystem,
    TransNeptunianRegion,
    FarthestRegions,
}

pub enum CelestialSubregion {
    InnerPlanets,
    AsteroidBelt,
    OuterPlanets,
    Centaurs,
    KuiperBelt,
    ScatteredDisc,
    DetachedObjects,
}

/// Returned from the data access layer, hence `Serialize`.
/// This is the "entity" that is used by the application. It maps to database table/s,
/// so sqlx's `FromRow` is implemented.
#[derive(Clone, Debug, Fields, FromRow, Serialize)]
pub struct CelestialBody {
    pub id: i64,
    pub name: String,
}

/// Sent to the data access layer, hence `Deserialize`.
#[derive(Fields, Deserialize)]
pub struct CelestialBodyCreate {
    pub name: String,
}

/// Sent to the data access layer, hence `Deserialize`.
#[derive(Fields, Deserialize)]
pub struct CelestialBodyUpdate {
    pub name: Option<String>,
}

// -----------------------------------------------------------------------------
// Client
// -----------------------------------------------------------------------------

pub struct Client;

impl DbCrudServer for Client {
    const TABLE: &'static str = "celestial_body";
}

impl Client {
    pub async fn create(
        ctx: &RequestContext,
        dam: &DataAccessManager,
        data: CelestialBodyCreate,
    ) -> Result<i64> {
        DbCrudAction::create::<Self, _>(ctx, dam, data).await
    }

    pub async fn read(
        ctx: &RequestContext,
        dam: &DataAccessManager,
        id: i64,
    ) -> Result<CelestialBody> {
        DbCrudAction::read::<Self, _>(ctx, dam, id).await
    }

    pub async fn read_all(
        ctx: &RequestContext,
        dam: &DataAccessManager,
    ) -> Result<Vec<CelestialBody>> {
        DbCrudAction::read_all::<Self, _>(ctx, dam).await
    }

    pub async fn delete(ctx: &RequestContext, dam: &DataAccessManager, id: i64) -> Result<()> {
        DbCrudAction::delete::<Self>(ctx, dam, id).await
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

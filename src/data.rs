use crate::AppResult;
///! Data access layer for the application.
///!
///! 1. Core database pool connection
///! 2. Core router & cross-cutting concerns
///! 3. Celestial bodies (e.g. planets, moons, asteroids, etc.)
///! 4. Celestial regions (e.g. inner solar system, outer solar system, etc.)
///! 5. Celestial subregions (e.g. inner planets, outer planets, etc.)
///!
use anyhow::Context;
use axum::extract::FromRef;
use serde::Deserialize;
use sqlx::sqlite::SqlitePool as DbPool;

// -------------------------------------------------------------------------------
// 1. Core database pool connection
// -------------------------------------------------------------------------------

/// SQLX provides database pooling; access within the application is provided by the application state.
/// This is a wrapper around the pool, simplifying access to resources within the handlers.
#[derive(Clone, FromRef)]
pub struct DatabaseConnection(pub DbPool);

impl DatabaseConnection {
    pub async fn new(db_url: &str) -> anyhow::Result<Self> {
        let db_connection = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(10)
            .connect(db_url)
            .await;

        match db_connection {
            Ok(db_pool) => Ok(Self(db_pool)),
            Err(e) => Err(e).context("Failed to connect to database"),
        }
    }

    pub fn celestial_bodies(&self) -> CelestialBodies {
        CelestialBodies(self.0.clone())
    }

    pub fn celestial_regions(&self) -> CelestialRegions {
        CelestialRegions(self.0.clone())
    }

    pub fn celestial_subregions(&self) -> CelestialSubregions {
        CelestialSubregions(self.0.clone())
    }
}

// -------------------------------------------------------------------------------
// 2. Core router & cross-cutting concerns
// -------------------------------------------------------------------------------

// -------------------------------------------------------------------------------
// 3. Celestial bodies (e.g. planets, moons, asteroids, etc.)
// -------------------------------------------------------------------------------

/// The `CelestialBody` struct represents a solar system object, such as a planet, moon, asteroid, etc.
#[derive(Deserialize)]
pub struct CelestialBody {
    pub body_name: String,
    pub radius: f64,
    pub aphelion: f64,
    pub perhelion: f64,
    pub orbital_period: f64,
    pub region: Option<i32>,
    pub subregion: Option<i32>,
    pub created_at: String,
    pub updated_at: Option<String>,
}

pub struct CelestialBodies(pub DbPool);

// -------------------------------------------------------------------------------
// 4. Celestial regions (e.g. inner solar system, outer solar system, etc.)
// -------------------------------------------------------------------------------

/// The `CelestialRegion` struct represents a region of the solar system, such as the inner solar system, the outer solar system, etc.
#[derive(Deserialize)]
pub struct CelestialRegion {
    pub region_id: i64,
    pub region_name: String,
    pub region_description: Option<String>,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}

pub struct CelestialRegions(pub DbPool);

impl CelestialRegions {
    pub async fn insert(
        &self,
        region: &CelestialRegion,
    ) -> AppResult<CelestialRegion, sqlx::Error> {
        let inserted = sqlx::query_as!(
            CelestialRegion,
          	"INSERT INTO celestial_region (region_name, created_at) VALUES (?1, strftime('%s', 'now')) RETURNING *",
            region.region_name
        )
        .fetch_one(&self.0)
        .await?;

        Ok(inserted)
    }

    pub async fn get_all(&self) -> AppResult<Vec<CelestialRegion>, sqlx::Error> {
        sqlx::query_as!(CelestialRegion, "SELECT * FROM celestial_region")
            .fetch_all(&self.0)
            .await
    }

    pub async fn get_by_id(&self, id: i64) -> AppResult<CelestialRegion, sqlx::Error> {
        sqlx::query_as!(
            CelestialRegion,
            "SELECT * FROM celestial_region WHERE region_id = ?1",
            id
        )
        .fetch_one(&self.0)
        .await
    }

    pub async fn update_name(&self, id: &str, new_name: &str) -> AppResult<i64, sqlx::Error> {
        let updated = sqlx::query!(
            "UPDATE celestial_region SET region_name = ?2, updated_at = strftime('%s', 'now') WHERE region_id = ?1",
            id,
            new_name
        )
        .execute(&self.0)
        .await?
        .last_insert_rowid();

        Ok(updated)
    }

    pub async fn update_description(&self, id: &str, desc: &str) -> AppResult<i64, sqlx::Error> {
        let updated = sqlx::query!(
            "UPDATE celestial_region SET region_description = ?2, updated_at = strftime('%s', 'now') WHERE region_id = ?1",
            id,
            desc
        )
        .execute(&self.0)
        .await?
        .last_insert_rowid();

        Ok(updated)
    }

    pub async fn delete(&self, id: i64) -> AppResult<u64, sqlx::Error> {
        let deleted = sqlx::query!("DELETE FROM celestial_region WHERE region_id = ?1", id)
            .execute(&self.0)
            .await?
            .rows_affected();

        Ok(deleted)
    }
}

// -------------------------------------------------------------------------------
// 5. Celestial subregions (e.g. inner planets, outer planets, etc.)
// -------------------------------------------------------------------------------

/// The `CelestialSubregion` struct represents a subregion of the solar system, such as the inner planets, the outer planets, etc.
#[derive(Deserialize)]
pub struct CelestialSubregion {
    pub subregion_id: i64,
    pub subregion_name: String,
    pub subregion_description: Option<String>,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}

pub struct CelestialSubregions(pub DbPool);

impl CelestialSubregions {
    pub async fn insert(
        &self,
        subregion: &CelestialSubregion,
    ) -> AppResult<CelestialSubregion, sqlx::Error> {
        let inserted = sqlx::query_as!(
            CelestialSubregion,
          	"INSERT INTO celestial_subregion (subregion_name, created_at) VALUES (?1, strftime('%s', 'now')) RETURNING *",
            subregion.subregion_name
        )
        .fetch_one(&self.0)
        .await?;

        Ok(inserted)
    }

    pub async fn get_all(&self) -> AppResult<Vec<CelestialSubregion>, sqlx::Error> {
        sqlx::query_as!(CelestialSubregion, "SELECT * FROM celestial_subregion")
            .fetch_all(&self.0)
            .await
    }

    pub async fn get_by_id(&self, id: i64) -> AppResult<CelestialSubregion, sqlx::Error> {
        sqlx::query_as!(
            CelestialSubregion,
            "SELECT * FROM celestial_subregion WHERE subregion_id = ?1",
            id
        )
        .fetch_one(&self.0)
        .await
    }

    pub async fn update_name(&self, id: &str, new_name: &str) -> AppResult<i64, sqlx::Error> {
        let updated = sqlx::query!(
            "UPDATE celestial_subregion SET subregion_name = ?2, updated_at = strftime('%s', 'now') WHERE subregion_id = ?1",
            id,
            new_name
        )
        .execute(&self.0)
        .await?
        .last_insert_rowid();

        Ok(updated)
    }

    pub async fn update_description(&self, id: &str, desc: &str) -> AppResult<i64, sqlx::Error> {
        let updated = sqlx::query!(
            "UPDATE celestial_subregion SET subregion_description = ?2, updated_at = strftime('%s', 'now') WHERE subregion_id = ?1",
            id,
            desc
        )
        .execute(&self.0)
        .await?
        .last_insert_rowid();

        Ok(updated)
    }

    pub async fn delete(&self, id: i64) -> AppResult<u64, sqlx::Error> {
        let deleted = sqlx::query!(
            "DELETE FROM celestial_subregion WHERE subregion_id = ?1",
            id
        )
        .execute(&self.0)
        .await?
        .rows_affected();

        Ok(deleted)
    }
}

use crate::{AppResult, AppState};
///! Data access layer for the application.
///!
///! 1. Core database pool connection
///! 2. Core router & cross-cutting concerns
///! 3. Celestial bodies (e.g. planets, moons, asteroids, etc.)
///! 4. Celestial regions (e.g. inner solar system, outer solar system, etc.)
///! 5. Celestial subregions (e.g. inner planets, outer planets, etc.)
///!
use anyhow::Context;
use axum::extract::{FromRef, Path, State};
use axum::routing::{delete, get, patch, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool as DbPool;

// -------------------------------------------------------------------------------
// 1. Core database pool connection
// -------------------------------------------------------------------------------

/// SQLX provides database pooling; access within the application is provided by the application state.
/// This is a wrapper around the pool, simplifying access to resources within the handlers.
#[derive(Clone, FromRef)]
pub struct DatabaseConnection(pub DbPool);

impl DatabaseConnection {
    pub async fn new(db_url: &str) -> AppResult<Self, anyhow::Error> {
        let db_connection = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(10)
            .connect(db_url)
            .await;

        match db_connection {
            Ok(db_pool) => Ok(Self(db_pool)),
            Err(e) => Err(e).context("Failed to connect to database"),
        }
    }
}

// -------------------------------------------------------------------------------
// 2. Core router & cross-cutting concerns
// -------------------------------------------------------------------------------

pub fn api_routes() -> Router<AppState> {
    axum::Router::new().nest("/api", region_routes())
}

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

// -------------------------------------------------------------------------------
// 4. Celestial regions (e.g. inner solar system, outer solar system, etc.)
// -------------------------------------------------------------------------------

fn region_routes() -> Router<AppState> {
    axum::Router::new()
        .route("/regions", post(create_region))
        .route("/regions", get(get_all_regions))
        .route("/regions/:region_id", get(get_region))
        // .route("/regions/:region_id", patch(update_region_name))
        // .route("/regions/:region_id", patch(update_region_description))
        .route("/regions/:region_id", delete(delete_region))
}

/// The `CelestialRegion` struct represents a region of the solar system, such as the inner solar system, the outer solar system, etc.
#[derive(Deserialize, Serialize)]
pub struct Region {
    pub region_id: i64,
    pub region_name: String,
    pub region_description: Option<String>,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}

#[derive(Deserialize)]
pub struct NewRegion {
    pub region_name: String,
    pub region_description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateRegionName {
    pub region_name: String,
}

#[derive(Deserialize)]
pub struct UpdateRegionDescription {
    pub region_description: String,
}

#[axum::debug_handler]
async fn create_region(
    State(app_state): State<AppState>,
    Json(new_region): Json<NewRegion>,
) -> AppResult<Json<Region>> {
    let inserted = sqlx::query_as!(
						Region,
          	"INSERT INTO celestial_region (region_name, region_description, created_at) VALUES (?1, ?2, strftime('%s', 'now')) RETURNING *",
            new_region.region_name,
						new_region.region_description
        )
        .fetch_one(&app_state.db_conn.0)
        .await?;

    Ok(Json(inserted))
}

#[axum::debug_handler]
async fn get_all_regions(State(app_state): State<AppState>) -> AppResult<Json<Vec<Region>>> {
    let regions = sqlx::query_as!(Region, "SELECT * FROM celestial_region")
        .fetch_all(&app_state.db_conn.0)
        .await?;

    Ok(Json(regions))
}

#[axum::debug_handler]
async fn get_region(
    State(app_state): State<AppState>,
    Path(region_id): Path<i64>,
) -> AppResult<Json<Region>> {
    let region = sqlx::query_as!(
        Region,
        "SELECT * FROM celestial_region WHERE region_id = ?1",
        region_id
    )
    .fetch_one(&app_state.db_conn.0)
    .await?;

    Ok(Json(region))
}

async fn update_region_name(
    State(app_state): State<AppState>,
    Path(region_id): Path<i64>,
    Json(update_region_name): Json<UpdateRegionName>,
) -> AppResult<Json<Region>> {
    unimplemented!()
    // let updated = sqlx::query_as!(
    // 		Region,
    // 		"UPDATE celestial_region SET region_name = ?2, updated_at = strftime('%s', 'now') WHERE region_id = ?1 RETURNING *",
    // 		region_id,
    // 		update_region_name.region_name
    // )
    // .fetch_one(&app_state.db_conn.0)
    // .await?;

    // Ok(Json(updated))
}

async fn update_region_description(
    State(app_state): State<AppState>,
    Path(region_id): Path<i64>,
    Json(update_region_description): Json<UpdateRegionDescription>,
) -> AppResult<Json<Region>> {
    unimplemented!()
    // let updated = sqlx::query_as!(
    // 		Region,
    // 		"UPDATE celestial_region SET region_description = ?2, updated_at = strftime('%s', 'now') WHERE region_id = ?1",
    // 		region_id,
    // 		update_region_description.region_description
    // )
    // .fetch_one(&app_state.db_conn.0)
    // .await?;

    // Ok(Json(updated))
}

async fn delete_region(
    State(app_state): State<AppState>,
    Path(region_id): Path<i64>,
) -> AppResult<Json<Region>> {
    let deleted = sqlx::query_as!(
        Region,
        "DELETE FROM celestial_region WHERE region_id = ?1 RETURNING *",
        region_id
    )
    .fetch_one(&app_state.db_conn.0)
    .await?;

    Ok(Json(deleted))
}

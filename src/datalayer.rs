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
use axum::routing::{delete, get, post};
use axum::{async_trait, Json, Router};
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
    axum::Router::new()
        // .nest("/api", body_routes())
        .nest("/api", region_routes())
        .nest("/api", subregion_routes())
}

// -------------------------------------------------------------------------------
// 3. Celestial bodies (e.g. planets, moons, asteroids, etc.)
// -------------------------------------------------------------------------------

// fn body_routes() -> Router<AppState> {
//     axum::Router::new()
//         // .route("/bodies", post(create_body))
//         .route("/bodies", get(get_all_bodies))
//         .route("/bodies/:body_id", get(get_body))
//         // .route("/bodies/:body_id", patch(update_body_name))
//         // .route("/bodies/:body_id", patch(update_body_description))
//         .route("/bodies/:body_id", delete(delete_body))
// }

/// The `CelestialBody` struct represents a solar system object, such as a planet, moon, asteroid, etc.
#[derive(Deserialize, Serialize)]
pub struct CelestialBody {
    pub body_id: i64,
    pub body_name: String,
    pub radius: f64,
    pub aphelion: f64,
    pub perihelion: f64,
    pub orbital_period: f64,
    pub region: Option<i64>,
    pub subregion: Option<i64>,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Deserialize)]
pub struct NewBody {
    pub body_name: String,
    pub radius: f64,
    pub aphelion: f64,
    pub perihelion: f64,
    pub orbital_period: f64,
    pub region: Option<i64>,
    pub subregion: Option<i64>,
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
#[derive(Debug, Deserialize, Serialize)]
pub struct Region {
    pub region_id: i64,
    pub region_name: String,
    pub region_description: Option<String>,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct NewRegion {
    pub region_name: String,
    pub region_description: Option<String>,
}

// #[derive(Deserialize)]
// pub struct UpdateRegionName {
//     pub region_name: String,
// }

// #[derive(Deserialize)]
// pub struct UpdateRegionDescription {
//     pub region_description: String,
// }

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

// -------------------------------------------------------------------------------
// 5. Celestial subregions (e.g. inner planets, outer planets, etc.)
// -------------------------------------------------------------------------------

fn subregion_routes() -> Router<AppState> {
    axum::Router::new()
        .route("/subregions", post(create_subregion))
        .route("/subregions", get(get_all_subregions))
        .route("/subregions/:subregion_id", get(get_subregion))
        // .route("/subregions/:subregion_id", patch(update_subregion_name))
        // .route("/subregions/:subregion_id", patch(update_subregion_description))
        .route("/subregions/:subregion_id", delete(delete_subregion))
}

/// The `Celestialsubregion` struct represents a subregion of the solar system, such as the inner solar system, the outer solar system, etc.
#[derive(Deserialize, Serialize)]
pub struct Subregion {
    pub subregion_id: i64,
    pub subregion_name: String,
    pub subregion_description: Option<String>,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}

#[derive(Deserialize)]
pub struct NewSubregion {
    pub subregion_name: String,
    pub subregion_description: Option<String>,
}

#[axum::debug_handler]
async fn create_subregion(
    State(app_state): State<AppState>,
    Json(new_subregion): Json<NewSubregion>,
) -> AppResult<Json<Subregion>> {
    let inserted = sqlx::query_as!(
						Subregion,
          	"INSERT INTO celestial_subregion (subregion_name, subregion_description, created_at) VALUES (?1, ?2, strftime('%s', 'now')) RETURNING *",
            new_subregion.subregion_name,
						new_subregion.subregion_description
        )
        .fetch_one(&app_state.db_conn.0)
        .await?;

    Ok(Json(inserted))
}

#[axum::debug_handler]
async fn get_all_subregions(State(app_state): State<AppState>) -> AppResult<Json<Vec<Subregion>>> {
    let subregions = sqlx::query_as!(Subregion, "SELECT * FROM celestial_subregion")
        .fetch_all(&app_state.db_conn.0)
        .await?;

    Ok(Json(subregions))
}

#[axum::debug_handler]
async fn get_subregion(
    State(app_state): State<AppState>,
    Path(subregion_id): Path<i64>,
) -> AppResult<Json<Subregion>> {
    let subregion = sqlx::query_as!(
        Subregion,
        "SELECT * FROM celestial_subregion WHERE subregion_id = ?1",
        subregion_id
    )
    .fetch_one(&app_state.db_conn.0)
    .await?;

    Ok(Json(subregion))
}

async fn delete_subregion(
    State(app_state): State<AppState>,
    Path(subregion_id): Path<i64>,
) -> AppResult<Json<Subregion>> {
    let deleted = sqlx::query_as!(
        Subregion,
        "DELETE FROM celestial_subregion WHERE subregion_id = ?1 RETURNING *",
        subregion_id
    )
    .fetch_one(&app_state.db_conn.0)
    .await?;

    Ok(Json(deleted))
}

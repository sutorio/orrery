use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions, SqliteRow};
use sqlx::FromRow;
use std::marker::PhantomData;
use std::sync::Arc;

// -----------------------------------------------------------------------------
// Models
// -----------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize)]
pub struct CelestialBody {
    /// The unique ID of the body; this is the primary key in the database, and is generated automatically.
    pub body_id: i64,
    /// The name of the body: eg "Earth", "Mars", "Jupiter", etc.
    pub body_name: String,
    /// A description of the body: eg "The third planet from the Sun, and the only planet known to support life."
    pub body_description: Option<String>,
    /// The radius of the body, in km.
    pub radius: f64,
    /// The distance from the Sun at which the body is furthest away, in km.
    pub aphelion: f64,
    /// The distance from the Sun at which the body is closest, in km.
    pub perihelion: f64,
    /// The time it takes for the body to complete one orbit of the Sun, in Earth days.
    pub orbital_period: f64,
    /// The ID of the region in which the body is located.
    pub region: Option<String>,
    /// The ID of the subregion in which the body is located.
    pub subregion: Option<String>,
    /// The date and time at which the body struct was first created, as a UNIX timestamp.
    pub created_at: u32,
    /// The date and time at which the body struct was last updated, as a UNIX timestamp.
    pub updated_at: Option<u32>,
}

impl<'c> FromRow<'c, SqliteRow> for CelestialBody {
    fn from_row(row: &'c SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            body_id: row.get("body_id"),
            body_name: row.get("body_name"),
            body_description: row.get("body_description"),
            radius: row.get("radius"),
            aphelion: row.get("aphelion"),
            perihelion: row.get("perihelion"),
            orbital_period: row.get("orbital_period"),
            region: row.get("region"),
            subregion: row.get("subregion"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CelestialRegion {
    /// The unique ID of the region; this is the primary key in the database, and is generated automatically.
    pub region_id: i64,
    /// The name of the region: eg "Inner Solar System", "Outer Solar System", etc.
    pub region_name: String,
    /// A description of the region: eg "The region of the solar system between the Sun and the asteroid belt."
    pub region_description: Option<String>,
    /// The date and time at which the region struct was first created, as a UNIX timestamp.
    pub created_at: u32,
    /// The date and time at which the region struct was last updated, as a UNIX timestamp.
    pub updated_at: Option<u32>,
}

impl<'c> FromRow<'c, SqliteRow> for CelestialRegion {
    fn from_row(row: &'c SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            region_id: row.get("region_id"),
            region_name: row.get("region_name"),
            region_description: row.get("region_description"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CelestialSubregion {
    /// The unique ID of the subregion; this is the primary key in the database, and is generated automatically.
    pub subregion_id: i64,
    /// The name of the subregion: eg "Inner Solar System", "Outer Solar System", etc.
    pub subregion_name: String,
    /// A description of the subregion: eg "The region of the solar system between the Sun and the asteroid belt."
    pub subregion_description: Option<String>,
    /// The date and time at which the subregion struct was first created, as a UNIX timestamp.
    pub created_at: u32,
    /// The date and time at which the subregion struct was last updated, as a UNIX timestamp.
    pub updated_at: Option<u32>,
}

impl FromRow<'c, SqliteRow> for CelestialSubregion {
    fn from_row(row: &'c SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            subregion_id: row.get("subregion_id"),
            subregion_name: row.get("subregion_name"),
            subregion_description: row.get("subregion_description"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

// -----------------------------------------------------------------------------
// Database connection setup and abstraction
// -----------------------------------------------------------------------------

pub struct Database<'c> {
    pub celestial_body: Arc<Table<'c, CelestialBody>>,
    pub celestial_region: Arc<Table<'c, CelestialRegion>>,
    pub celestial_subregion: Arc<Table<'c, CelestialSubregion>>,
}

impl<'a> Database<'a> {
    pub async fn new(db_url: &str) -> Database<'a> {
        let connection = SqlitePoolOptions::new()
            .max_connections(10)
            .connect(db_url)
            .await
            .expect("Failed to connect to database");

        Database {
            celestial_body: Arc::new(Table::new(Arc::new(connection.clone()))),
            celestial_region: Arc::new(Table::new(Arc::new(connection.clone()))),
            celestial_subregion: Arc::new(Table::new(Arc::new(connection.clone()))),
        }
    }
}

pub struct Table<'c, T>
where
    T: FromRow<'c, SqliteRow>,
{
    pub pool: Arc<SqlitePool>,
    _from_row: fn(&'c SqliteRow) -> Result<T, sqlx::Error>,
    _marker: PhantomData<&'c T>,
}

impl<'c, T> Table<'c, T>
where
    T: FromRow<'c, SqliteRow>,
{
    fn new(pool: Arc<SqlitePool>) -> Self {
        Self {
            pool,
            _from_row: T::from_row,
            _marker: PhantomData,
        }
    }
}

// -----------------------------------------------------------------------------
// Concrete data access methods
// -----------------------------------------------------------------------------

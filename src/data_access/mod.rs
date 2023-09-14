//! Data access layer for the application.
//!
//! Rules:
//! 1. The model layer normalisesz the application's data structures and data access.
//! 2. All applicaiton data must go through the model layer.
//! 3. The `DataModelManager` holds references to the internal states/resources required
//!    by the `DataModelControllers` to access data, for example the database connection pool.
//! 4. The `DataModelControllers` implement CRUD and other data access methods on "entities".
//! 5. In frameworks like Axum, the manager is used as the app state.
//! 6. The manager is designed to be passed as an argument to all controller functions.
mod error;
mod model;
mod store;

pub use self::error::{Error, Result};
use store::db::{create_database_pool, DbPool};

// -----------------------------------------------------------------------------
//
// -----------------------------------------------------------------------------

#[derive(Clone)]
pub struct DataAccessManager {
    db_pool: DbPool,
}

impl DataAccessManager {
    /// Standard constructor
    pub async fn new() -> Result<Self> {
        let db_pool = create_database_pool().await?;

        Ok(DataAccessManager { db_pool })
    }

    pub async fn new_from_existing_resources(db_pool: DbPool) -> Result<Self> {
        Ok(DataAccessManager { db_pool })
    }

    // NOTE: the `pub(in crate::data_model)` syntax is used to make the method
    //       public within the crate, but private outside of it.
    pub(in crate::data_access) fn db_pool(&self) -> &DbPool {
        &self.db_pool
    }
}

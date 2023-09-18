use crate::data_access::{DataAccessManager, DbCrudAction, DbCrudServer, Error, Result};
use crate::RequestContext;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::FromRow;

// -----------------------------------------------------------------------------
// Types
// -----------------------------------------------------------------------------
const TABLE_NAME: &'static str = "user";

/// Sent from the server to the client
#[derive(Clone, Debug, FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

/// Sent from client to server (or via an API)
#[derive(Debug, Deserialize)]
pub struct UserCreate {
    pub username: String,
    pub pwd_clear: String,
}

/// Internal, second step of a user creation. This is an implementation detail.
#[derive(Debug, Deserialize)]
struct UserInsert {
    username: String,
}

/// Read-only, the information necessary to validate the user.
#[derive(Clone, FromRow, Debug)]
pub struct UserLogin {
    pub id: i64,
    pub username: String,
    pub pwd: Option<String>,
    pub pwd_salt: String,
    pub token_salt: String, // encrypted password
}

/// Read-only, the information necessary to validate the user.
#[derive(Clone, FromRow, Debug)]
pub struct UserAuth {
    pub id: i64,
    pub username: String,
    pub token_salt: String, // encrypted password
}

// -----------------------------------------------------------------------------
// Server
//
// TODO: this should be generalised into a trait + generic functions, and then
//       implemented for each entity. This *should* allow the `Client` to just
//       call the generic functions. However, this is currently difficult, see
//       the notes in `src/data_access/mod.rs`.
// -----------------------------------------------------------------------------

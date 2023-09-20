use crate::data_access::{DataAccessManager, DbCrudAction, DbCrudServer, Error, Result};
use crate::{RequestContext, security};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::FromRow;

// -----------------------------------------------------------------------------
// Types/traits
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
    pub token_salt: String,
}

/// Read-only, subset of the `UserLogin`.
#[derive(Clone, FromRow, Debug)]
pub struct UserAuth {
    pub id: i64,
    pub username: String,
    pub token_salt: String,
}

/// Marker trait. This, by itself, does very little, but it allows the `User`
/// related CRUD functions to easily use any of the `User{ACTION}` structs.
pub trait UserBy: for<'r> FromRow<'r, SqliteRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserLogin {}
impl UserBy for UserAuth {}

// -----------------------------------------------------------------------------
// Server
//
// TODO: this should be generalised into a trait + generic functions, and then
//       implemented for each entity. This *should* allow the `Client` to just
//       call the generic functions. However, this is currently difficult, see
//       the notes in `src/data_access/mod.rs`.
// -----------------------------------------------------------------------------

pub struct Server;

impl DbCrudServer for Server {
    const TABLE: &'static str = "user";
}

impl Server {
    pub async fn fetch_by_id<E>(ctx: &RequestContext, dam: &DataAccessManager, id: i64) -> Result<E>
    where
        E: UserBy,
    {
        DbCrudAction::read::<Self, _>(ctx, dam, id).await
    }

    pub async fn fetch_by_username<E>(
        ctx: &RequestContext,
        dam: &DataAccessManager,
        username: &str,
    ) -> Result<E>
    where
        E: UserBy,
    {
        let db = dam.db_pool();
        let sql = format!("SELECT * FROM {} WHERE username = $1", TABLE_NAME);

        let user = sqlx::query_as(&sql).bind(username).fetch_one(db).await?;

        Ok(user)
    }

    pub async fn update_password<E>(
        ctx: &RequestContext,
        dam: &DataAccessManager,
        id: i64,
        password_clear: &str,
    ) -> Result<()> {
        let db = dam.db_pool();
        let user: UserLogin = Self::fetch_by_id(ctx, dam, id).await?;
        let password = security::encrypt_password(&security::EncryptedContent {
            content: password_clear.to_string(),
            salt: user.pwd_salt.to_string(),. 
        })?;
        let sql = format!("UPDATE {} SET pwd = $1 WHERE id = $2", TABLE_NAME);

        sqlx::query(&sql)
            .bind(&password.to_string())
            .bind(id)
            .execute(db)
            .await?;

        Ok(())
    }
}

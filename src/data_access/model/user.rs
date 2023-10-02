use crate::data_access::{DataAccessManager, DbCrudAction, DbCrudServer, Result};
use crate::{security, RequestContext};
use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields};
use sqlx::sqlite::SqliteRow;
use sqlx::FromRow;

// -----------------------------------------------------------------------------
// Types/traits
// -----------------------------------------------------------------------------
const TABLE_NAME: &'static str = "user";

/// Sent from the server to the client
#[derive(Clone, Debug, Fields, FromRow, Serialize)]
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
#[derive(Debug, Deserialize, Fields)]
pub struct UserInsert {
    pub username: String,
}

/// Read-only, the information necessary to validate the user.
#[derive(Clone, Fields, FromRow, Debug)]
pub struct UserLogin {
    pub id: i64,
    pub username: String,
    pub pwd: Option<String>,
    pub pwd_salt: String,
    pub token_salt: String,
}

/// Read-only, subset of the `UserLogin`.
#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserAuth {
    pub id: i64,
    pub username: String,
    pub token_salt: String,
}

/// Marker trait. This, by itself, does very little, but it allows the `User`
/// related CRUD functions to easily use any of the `User{ACTION}` structs.
pub trait UserBy: HasFields + for<'r> FromRow<'r, SqliteRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserLogin {}
impl UserBy for UserAuth {}

// -----------------------------------------------------------------------------
// Server
// -----------------------------------------------------------------------------

pub struct Server;

impl DbCrudServer for Server {
    const TABLE: &'static str = "user";
}

impl Server {
    pub async fn read<E>(ctx: &RequestContext, dam: &DataAccessManager, id: i64) -> Result<E>
    where
        E: UserBy,
    {
        DbCrudAction::read::<Self, _>(ctx, dam, id).await
    }

    pub async fn read_by_username<E>(
        _ctx: &RequestContext,
        dam: &DataAccessManager,
        username: &str,
    ) -> Result<Option<E>>
    where
        E: UserBy,
    {
        let user = sqlb::select()
            .table(TABLE_NAME)
            .and_where("username", "=", username)
            .fetch_optional::<_, E>(dam.db_pool())
            .await?;

        Ok(user)
    }

    pub async fn update_password<E>(
        ctx: &RequestContext,
        dam: &DataAccessManager,
        id: i64,
        cleartext_password: &str,
    ) -> Result<()> {
        let user: UserLogin = Self::read(ctx, dam, id).await?;
        let password = security::encrypt_password(&security::EncryptedContent {
            content: cleartext_password.to_string(),
            salt: user.pwd_salt.to_string(),
        })?;

        sqlb::update()
            .table(Self::TABLE)
            .and_where("id", "=", id)
            .data(vec![("pwd", password.to_string()).into()])
            .exec(dam.db_pool())
            .await?;

        Ok(())
    }
}

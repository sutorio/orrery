use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::security;

// -----------------------------------------------------------------------------
// Error handling
// -----------------------------------------------------------------------------

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    EntityNotFound { entity: &'static str, id: i64 },
    // Db-related errors
    FailedToCreatePool(String),
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
    // Wrapped errors
    Security(security::Error),
}

impl From<security::Error> for Error {
    fn from(err: security::Error) -> Self {
        Self::Security(err)
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::Sqlx(err)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

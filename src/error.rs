use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;

// Expose AppError's associated result type to the rest of the application.
pub type AppResult<T, E = AppError> = Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    /// 404 Not Found Error. A resource can't be found to fulfill the request
    #[error("request path not found")]
    NotFound,

    /// 500 Internal Server Error for a Serde JSON error.
    ///
    /// This allows using `?` on database calls in handler functions without a manual mapping step.
    ///
    /// The actual error message isn't returned to the client for security reasons.
    /// It should be logged instead.
    #[error("an error occurred with serde JSON de/serialisation")]
    SerdeJson(#[from] serde_json::Error),

    /// 500 Internal Server Error for a `rusqlite::Error`.
    ///
    /// This allows using `?` on database calls in handler functions without a manual mapping step.
    ///
    /// The actual error message isn't returned to the client for security reasons.
    /// It should be logged instead.
    ///
    /// Note that this could also contain database constraint errors, which should usually
    /// be transformed into client errors (e.g. `422 Unprocessable Entity` or `409 Conflict`).
    /// See https://github.com/davidpdrsn/realworld-axum-sqlx/blob/main/src/http/error.rs
    #[error("an error occurred with the database")]
    Sqlx(#[from] sqlx::Error),

    /// 500 Internal Server Error for a `anyhow::Error`.
    ///
    /// `anyhow::Error` is used in a few places to capture context and backtraces
    /// on unrecoverable (but technically non-fatal) errors which could be highly useful for
    /// debugging, for example for background tasks or making API calls
    /// to external services so `.context()` can be used to refine the logged error.
    ///
    /// The actual error message isn't returned to the client for security reasons.
    /// It should be logged instead.
    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::SerdeJson(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Sqlx(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// Axum allows you to return `Result` from handler functions, but the error type
/// also must be some sort of response type.
///
/// By default, the generated `Display` impl is used to return a plaintext error message
/// to the client.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::SerdeJson(ref e) => {
                error!("Serde error: {:?}", e);
            }
            Self::Sqlx(ref e) => {
                error!("Sqlx error: {:?}", e);
            }
            Self::Anyhow(ref e) => {
                error!("Generic error: {:?}", e);
            }
            _ => (),
        }

        (self.status_code(), self.to_string()).into_response()
    }
}

mod _dev_utils;
pub mod config;
pub mod data_access;
mod request_context;
pub mod security;

// -----------------------------------------------------------------------------
// Re-exports
// -----------------------------------------------------------------------------

pub use _dev_utils::initialise_development_environment;
pub use request_context::RequestContext;

// -----------------------------------------------------------------------------
// Top-level errors
//
// Errors are based on Enums, automatically deriving *at least* the `Debug` trait.
//
// They also must implement `Display`, to allow `to_string()` to be called on them.
// This uis because all of the errors will be logged, and the standard format will
// be JSON.
//
// For the command-line, something like `thiserror` could be used, but for logging from
// the backend on a production service, structured logging sent to a service like
// DataDog will need to be in JSON format.
// -----------------------------------------------------------------------------

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

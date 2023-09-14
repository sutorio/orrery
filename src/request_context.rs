use serde::Serialize;
// -----------------------------------------------------------------------------
// Error handling
// -----------------------------------------------------------------------------
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    CannotUseRootContext,
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// -----------------------------------------------------------------------------
// Implementation
// -----------------------------------------------------------------------------
pub struct RequestContext {
    user_id: i64,
}

impl RequestContext {
    // -------------------------------------------------------------------------
    // Constructors
    // -------------------------------------------------------------------------

    /// The *root* context is used internally by the system.
    /// There can only be one root contest, the id *must* be 0, and no other
    /// user can have this id.
    pub fn root_context() -> Self {
        RequestContext { user_id: 0 }
    }

    pub fn new(user_id: i64) -> Result<Self> {
        if user_id == 0 {
            Err(Error::CannotUseRootContext)
        } else {
            Ok(Self { user_id })
        }
    }

    // -------------------------------------------------------------------------
    // Property accessors
    // -------------------------------------------------------------------------
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}

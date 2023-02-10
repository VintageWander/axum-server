use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    // Internal error
    #[error("DB error")]
    Mongo(#[from] mongodb::error::Error),

    // Validation error
    #[error("Input fields error")]
    Fields(#[from] validator::ValidationErrors),

    #[error("Input field error")]
    Field(#[from] validator::ValidationError),

    // Library error

    // Custom error
    #[error("Resource not found")]
    ResourceNotFound,

    #[error("Conflict user")]
    ConflictUser,
}

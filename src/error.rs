use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::{helper::print_validation::extract_validation_error, web::Web};

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

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            // Internal error
            Error::Mongo(e) => Web::internal_error("MongoDB error", e),

            // Validation error
            Error::Fields(e) => Web::bad_request(
                "Some of the fields are incorrect",
                extract_validation_error(&e),
            ),
            Error::Field(e) => Web::bad_request("One of the field are incorrect", e),

            // Custom error
            Error::ResourceNotFound => Web::not_found(
                "Resource not found",
                "Users, files, folders, or any resource with the provided information could not be found",
            ),
            Error::ConflictUser => Web::conflict("Resource conflict", "This could mean that the resource provided already exists in the database"),
        }
    }
}

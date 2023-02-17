use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::{helper::print_validation::extract_validation_error, web::Web};

#[derive(Debug, Error)]
pub enum Error {
    // Internal error
    #[error("DB error")]
    Mongo(#[from] mongodb::error::Error),

    #[error("JWT error")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Infallible error")]
    Infallible(#[from] std::convert::Infallible),

    // Validation error
    #[error("Input fields error")]
    Fields(#[from] validator::ValidationErrors),

    #[error("Input field error")]
    Field(#[from] validator::ValidationError),

    // Bad requests error
    #[error("Invalid json form")]
    InvalidJson(#[from] axum::extract::rejection::JsonRejection),

    #[error("Invalid query string")]
    InvalidQuery(#[from] axum::extract::rejection::QueryRejection),

    #[error("Invalid path string form")]
    InvalidPath(#[from] axum::extract::rejection::PathRejection),

    #[error("Invalid id entered")]
    InvalidId(#[from] mongodb::bson::oid::Error),

    // Custom error
    #[error("Resource not found")]
    ResourceNotFound,

    #[error("Conflict user")]
    ConflictUser,

    #[error("Passwords mismatch")]
    PasswordsMismatch,

    #[error("Invalid Password")]
    InvalidPassword,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Split error")]
    Split,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            // Internal error
            Error::Mongo(e) => Web::internal_error("MongoDB error", e),
            Error::Jwt(e) => Web::internal_error("JWT error occur", e),
            Error::Infallible(e) => Web::internal_error("Infallible error", "If you see this error please let me know"),
            
            // Validation error
            Error::Fields(e) => Web::bad_request(
                "Some of the fields are incorrect",
                extract_validation_error(&e),
            ),
            Error::Field(e) => Web::bad_request("One of the field are incorrect", e),
            
            // Bad requests error
            Error::InvalidJson(e) => Web::bad_request("Invalid json form", e),
            Error::InvalidQuery(e) => Web::bad_request("Invalid query form", e),
            Error::InvalidId(e) => Web::bad_request("The id provided is invalid", e),
            Error::InvalidPath(e) => Web::bad_request("Invalid path string provided", e),

            // Custom error
            Error::ResourceNotFound => Web::not_found(
                "Resource not found",
                "Users, files, folders, or any resource with the provided information could not be found",
            ),
            Error::ConflictUser => Web::conflict("Resource conflict", "This could mean that the resource provided already exists in the database"),
            Error::PasswordsMismatch => Web::bad_request("Passwords mismatch", "Password and confirm password fields are incorrect"),
            Error::InvalidPassword => Web::bad_request("Invalid password", "The password provided does not match with the user's password in the database"),
            Error::Unauthorized => Web::forbidden("Unauthorized", "You are not logged in"),
            Error::Split => Web::internal_error("Split error", "Cannot split the full filename to filename and extension"),
        }
    }
}

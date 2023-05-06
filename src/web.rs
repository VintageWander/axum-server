use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

use crate::error::Error;

#[derive(Debug, Serialize)]
pub struct Web {
    code: String,
    message: String,
    data: Value,
    error: String,
}

impl Web {
    pub fn ok(
        message: impl ToString,
        data: impl Serialize + DeserializeOwned,
    ) -> Result<Response, Error> {
        Ok((
            StatusCode::OK,
            Json(Web {
                code: StatusCode::OK.to_string(),
                message: message.to_string(),
                data: json!(&data),
                error: "".into(),
            }),
        )
            .into_response())
    }

    pub fn created(
        message: impl ToString,
        data: impl Serialize + DeserializeOwned,
    ) -> Result<Response, Error> {
        Ok((
            StatusCode::CREATED,
            Json(Web {
                code: StatusCode::CREATED.to_string(),
                message: message.to_string(),
                data: json!(&data),
                error: "".into(),
            }),
        )
            .into_response())
    }

    pub fn forbidden(message: impl ToString, error: impl ToString) -> Result<Response, Error> {
        Ok((
            StatusCode::FORBIDDEN,
            Json(Web {
                code: StatusCode::FORBIDDEN.to_string(),
                message: message.to_string(),
                data: json!(()),
                error: error.to_string(),
            }),
        )
            .into_response())
    }

    pub fn conflict(message: impl ToString, error: impl ToString) -> Result<Response, Error> {
        Ok((
            StatusCode::CONFLICT,
            Json(Web {
                code: StatusCode::CONFLICT.to_string(),
                message: message.to_string(),
                data: json!(()),
                error: error.to_string(),
            }),
        )
            .into_response())
    }

    pub fn bad_request(message: impl ToString, error: impl ToString) -> Result<Response, Error> {
        Ok((
            StatusCode::BAD_REQUEST,
            Json(Web {
                code: StatusCode::BAD_REQUEST.to_string(),
                message: message.to_string(),
                data: json!(()),
                error: error.to_string(),
            }),
        )
            .into_response())
    }

    pub fn not_found(message: impl ToString, error: impl ToString) -> Result<Response, Error> {
        Ok((
            StatusCode::NOT_FOUND,
            Json(Web {
                code: StatusCode::NOT_FOUND.to_string(),
                message: message.to_string(),
                data: json!(()),
                error: error.to_string(),
            }),
        )
            .into_response())
    }

    pub fn internal_error(message: impl ToString, error: impl ToString) -> Result<Response, Error> {
        Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Web {
                code: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                message: message.to_string(),
                data: json!(()),
                error: error.to_string(),
            }),
        )
            .into_response())
    }
}

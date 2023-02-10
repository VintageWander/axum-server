use std::str::FromStr;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize)]
pub struct Web {
    code: String,
    message: String,
    data: Value,
    error: String,
}

impl IntoResponse for Web {
    fn into_response(self) -> Response {
        (StatusCode::from_str(&self.code).unwrap(), Json(self)).into_response()
    }
}

impl Web {
    pub fn ok(message: impl ToString, data: impl Serialize + DeserializeOwned) -> Response {
        Web {
            code: StatusCode::OK.to_string(),
            message: message.to_string(),
            data: json!(&data),
            error: "".into(),
        }
        .into_response()
    }

    pub fn created<T: Serialize + DeserializeOwned>(
        message: impl ToString,
        data: impl Serialize + DeserializeOwned,
    ) -> Response {
        Web {
            code: StatusCode::CREATED.to_string(),
            message: message.to_string(),
            data: json!(&data),
            error: "".into(),
        }
        .into_response()
    }

    pub fn forbidden(message: impl ToString, error: impl ToString) -> Response {
        Web {
            code: StatusCode::FORBIDDEN.to_string(),
            message: message.to_string(),
            data: json!(()),
            error: error.to_string(),
        }
        .into_response()
    }

    pub fn conflict(message: impl ToString, error: impl ToString) -> Response {
        Web {
            code: StatusCode::CONFLICT.to_string(),
            message: message.to_string(),
            data: json!(()),
            error: error.to_string(),
        }
        .into_response()
    }

    pub fn bad_request(message: impl ToString, error: impl ToString) -> Response {
        Web {
            code: StatusCode::BAD_REQUEST.to_string(),
            message: message.to_string(),
            data: json!(()),
            error: error.to_string(),
        }
        .into_response()
    }

    pub fn not_found(message: impl ToString, error: impl ToString) -> Response {
        Web {
            code: StatusCode::NOT_FOUND.to_string(),
            message: message.to_string(),
            data: json!(()),
            error: error.to_string(),
        }
        .into_response()
    }

    pub fn internal_error(message: impl ToString, error: impl ToString) -> Response {
        Web {
            code: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
            message: message.to_string(),
            data: json!(()),
            error: error.to_string(),
        }
        .into_response()
    }
}

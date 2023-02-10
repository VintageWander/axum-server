use std::str::FromStr;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::Value;

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
    pub fn ok(message: impl ToString, data: Value) -> Response {
        Web {
            code: StatusCode::OK.to_string(),
            message: message.to_string(),
            data,
            error: "".into(),
        }
        .into_response()
    }
}

use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use serde::{Deserialize, Serialize};

use crate::{error::Error, services::Service};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileRestoreRequest {
    pub file_id: String,
    pub restore_version: i64,
}

#[async_trait]
impl FromRequest<Service, Body> for FileRestoreRequest {
    type Rejection = Error;
    async fn from_request(req: Request<Body>, state: &Service) -> Result<Self, Self::Rejection> {
        let Json(restore_req) = Json::<FileRestoreRequest>::from_request(req, state).await?;
        Ok(restore_req)
    }
}

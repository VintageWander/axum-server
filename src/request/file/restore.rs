use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use serde::{Deserialize, Serialize};

use crate::{error::Error, SharedState};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileRestoreRequest {
    pub file_id: String,
    pub restore_version: i64,
}

#[async_trait]
impl FromRequest<SharedState, Body> for FileRestoreRequest {
    type Rejection = Error;
    async fn from_request(
        req: Request<Body>,
        state: &SharedState,
    ) -> Result<Self, Self::Rejection> {
        let Json(restore_req) = Json::<FileRestoreRequest>::from_request(req, state).await?;
        Ok(restore_req)
    }
}

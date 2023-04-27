use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};

use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    error::Error,
    model::{folder::Folder, user::User},
    service::Service,
    validation::file::*,
    Result,
};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderRequest {
    #[validate(custom = "check_folder_name")]
    pub folder_name: String,

    #[validate(custom = "check_dir")]
    pub position: String,

    #[validate(custom = "check_visibility")]
    pub visibility: String,
}

#[async_trait]
impl FromRequest<Service, Body> for CreateFolderRequest {
    type Rejection = Error;
    async fn from_request(
        req: Request<Body>,
        state: &Service,
    ) -> std::result::Result<Self, Self::Rejection> {
        let Json(folder_req) = Json::<CreateFolderRequest>::from_request(req, state).await?;

        folder_req.validate()?;

        Ok(folder_req)
    }
}

impl CreateFolderRequest {
    pub fn into_folder_with_owner(self, owner: &User) -> Result<Folder> {
        Folder::new(
            ObjectId::new(),
            owner,
            self.folder_name,
            self.position,
            self.visibility.try_into()?,
            Utc::now().timestamp_millis(),
        )
    }
}

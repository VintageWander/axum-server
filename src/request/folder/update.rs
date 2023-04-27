use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};

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
pub struct UpdateFolderRequest {
    #[validate(custom = "check_folder_name")]
    pub folder_name: String,

    #[validate(custom = "check_dir")]
    pub position: String,

    #[validate(custom = "check_visibility")]
    pub visibility: String,
}

#[async_trait]
impl FromRequest<Service, Body> for UpdateFolderRequest {
    type Rejection = Error;
    async fn from_request(
        req: Request<Body>,
        state: &Service,
    ) -> std::result::Result<Self, Self::Rejection> {
        let Json(folder_req) = Json::<UpdateFolderRequest>::from_request(req, state).await?;

        folder_req.validate()?;

        Ok(folder_req)
    }
}

impl UpdateFolderRequest {
    pub fn into_folder_with_owner(self, old_folder: Folder, owner: &User) -> Result<Folder> {
        Folder::new(
            old_folder.id,
            owner,
            self.folder_name,
            self.position,
            self.visibility.try_into()?,
            old_folder.created_at,
        )
    }
}

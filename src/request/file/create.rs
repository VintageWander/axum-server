use axum::{
    async_trait,
    body::Body,
    extract::{FromRequest, Multipart},
    http::Request,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{error::Error, model::file::FileVisibility, validation::file::*, SharedState};

#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct CreateFileRequest {
    #[validate(custom = "check_full_filename")]
    pub filename: String,

    #[validate(custom = "check_dir")]
    pub position: String,

    pub visility: FileVisibility,

    pub file: Vec<u8>,
}

#[async_trait]
impl FromRequest<SharedState, Body> for CreateFileRequest {
    type Rejection = Error;
    async fn from_request(
        req: Request<Body>,
        state: &SharedState,
    ) -> Result<Self, Self::Rejection> {
        let mut multipart = Multipart::from_request(req, state).await?;

        let mut file_req = CreateFileRequest::default();

        while let Some(field) = multipart.next_field().await? {
            if let Some(filename) = field.file_name() {
                file_req.filename = filename.to_owned();
                file_req.file = field.bytes().await?.to_vec().to_owned();
            } else if let Some(key) = field.name() {
                match key {
                    "position" => file_req.position = field.text().await?.to_owned(),
                    "visibility" => {
                        file_req.visility = field.text().await?.to_owned().try_into()?
                    }
                    _ => continue,
                }
            }
        }

        file_req.validate()?;

        Ok(file_req)
    }
}

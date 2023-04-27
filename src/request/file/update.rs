use axum::{
    async_trait,
    body::Body,
    extract::{FromRequest, Multipart},
    http::Request,
};

use chrono::Utc;
use serde::{Deserialize, Serialize};

use validator::Validate;

use crate::{
    error::Error,
    model::{
        file::{File, FileVisibility},
        user::User,
    },
    service::Service,
    validation::file::*,
    Result,
};

#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct UpdateFileRequest {
    #[validate(custom = "check_full_filename")]
    pub filename: Option<String>,

    #[validate(custom = "check_dir")]
    pub position: Option<String>,

    pub visility: Option<FileVisibility>,

    pub file: Option<Vec<u8>>,
}

#[async_trait]
impl FromRequest<Service, Body> for UpdateFileRequest {
    type Rejection = Error;
    async fn from_request(
        req: Request<Body>,
        state: &Service,
    ) -> std::result::Result<Self, Self::Rejection> {
        let mut multipart = Multipart::from_request(req, state).await?;

        let mut file_req = UpdateFileRequest::default();

        while let Some(field) = multipart.next_field().await? {
            if let Some(filename) = field.file_name() {
                file_req.filename = Some(filename.to_owned());
                file_req.file = Some(field.bytes().await?.to_vec().to_owned());
            } else if let Some(key) = field.name() {
                match key {
                    "position" => file_req.position = Some(field.text().await?.to_owned()),
                    "visibility" => {
                        file_req.visility = Some(field.text().await?.to_owned().try_into()?)
                    }
                    _ => continue,
                }
            }
        }

        file_req.validate()?;

        Ok(file_req)
    }
}

impl UpdateFileRequest {
    pub fn into_file(self, owner: &User, old_file: File) -> Result<(File, Vec<u8>)> {
        Ok((
            File::new(
                old_file.id,
                owner,
                self.filename.unwrap_or(old_file.full_filename),
                self.visility.unwrap_or(old_file.visibility),
                self.position.unwrap_or(old_file.position),
                Utc::now().timestamp_millis(),
            )?,
            self.file.unwrap_or_default(),
        ))
    }
}

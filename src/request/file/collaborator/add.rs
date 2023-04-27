use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{error::Error, service::Service};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct FileAddCollaboratorRequest {
    #[validate(email(message = "Invalid email"))]
    pub email: String,
}

#[async_trait]
impl FromRequest<Service, Body> for FileAddCollaboratorRequest {
    type Rejection = Error;
    async fn from_request(req: Request<Body>, state: &Service) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<FileAddCollaboratorRequest>::from_request(req, state).await?;

        body.validate()?;

        if !state.exists_user_by_email(&body.email).await? {
            return Err(Error::NotFound);
        }

        Ok(body)
    }
}

use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{error::Error, service::Service, validation::user::check_password};

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUserRequest {
    #[validate(custom = "check_password")]
    pub password: String,

    #[validate(custom = "check_password")]
    pub confirm_password: String,
}

#[async_trait]
impl FromRequest<Service, Body> for DeleteUserRequest {
    type Rejection = Error;
    async fn from_request(req: Request<Body>, state: &Service) -> Result<Self, Self::Rejection> {
        let Json(user_req) = Json::<DeleteUserRequest>::from_request(req, state).await?;

        user_req.validate()?;

        if user_req.password != user_req.confirm_password {
            return Err(Error::PasswordsMismatch);
        }

        Ok(user_req)
    }
}

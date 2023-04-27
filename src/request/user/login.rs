use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    error::Error,
    service::Service,
    validation::user::{check_password, check_username},
};

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginUserRequest {
    #[validate(custom = "check_username")]
    pub username: String,

    #[validate(custom = "check_password")]
    pub password: String,
}

#[async_trait]
impl FromRequest<Service, Body> for LoginUserRequest {
    type Rejection = Error;

    async fn from_request(req: Request<Body>, state: &Service) -> Result<Self, Self::Rejection> {
        let Json(user_req) = Json::<LoginUserRequest>::from_request(req, state).await?;

        user_req.validate()?;

        Ok(user_req)
    }
}

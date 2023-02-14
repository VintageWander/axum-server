use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    error::Error,
    model::user::User,
    validation::user::{check_password, check_username},
    SharedState,
};

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    #[validate(custom = "check_username")]
    username: String,

    #[validate(email(message = "Email is invalid"))]
    email: String,

    #[validate(custom = "check_password")]
    password: String,

    #[validate(custom = "check_password")]
    confirm_password: String,
}

#[async_trait]
impl FromRequest<SharedState, Body> for CreateUserRequest {
    type Rejection = Error;
    async fn from_request(
        req: Request<Body>,
        state: &SharedState,
    ) -> Result<Self, Self::Rejection> {
        let Json(user_req) = Json::<CreateUserRequest>::from_request(req, state).await?;
        user_req.validate()?;
        if user_req.password != user_req.confirm_password {
            return Err(Error::PasswordsMismatch);
        }
        Ok(user_req)
    }
}

impl TryFrom<CreateUserRequest> for User {
    type Error = Error;
    fn try_from(user_req: CreateUserRequest) -> Result<Self, Self::Error> {
        let user = User::builder()
            .username(user_req.username)
            .email(user_req.email)
            .password(user_req.password)
            .build()?;
        Ok(user)
    }
}

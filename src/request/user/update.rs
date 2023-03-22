use axum::{async_trait, body::Body, extract::FromRequest, http::Request, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    error::Error,
    model::user::User,
    services::Service,
    validation::user::{check_password, check_username},
    Result,
};

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequest {
    #[validate(custom = "check_username")]
    pub username: Option<String>,

    #[validate(email(message = "Email is invalid"))]
    pub email: Option<String>,

    #[validate(custom = "check_password")]
    pub password: String,

    #[validate(custom = "check_password")]
    pub new_password: Option<String>,

    #[validate(custom = "check_password")]
    pub confirm_new_password: Option<String>,
}

#[async_trait]
impl FromRequest<Service, Body> for UpdateUserRequest {
    type Rejection = Error;
    async fn from_request(
        req: Request<Body>,
        state: &Service,
    ) -> std::result::Result<Self, Self::Rejection> {
        let Json(user_req) = Json::<UpdateUserRequest>::from_request(req, state).await?;
        user_req.validate()?;
        Ok(user_req)
    }
}

impl UpdateUserRequest {
    pub fn into_user(self, old_user: User) -> Result<User> {
        User::builder()
            .id(old_user.id)
            .username(self.username.unwrap_or(old_user.username))
            .email(self.email.unwrap_or(old_user.email))
            .password(self.new_password.unwrap_or(old_user.password))
            .created_at(old_user.created_at)
            .build()
    }
}

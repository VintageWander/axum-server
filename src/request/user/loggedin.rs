use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;

use crate::{
    error::Error, helper::auth::decode::decode_access_token, model::user::User, services::Service,
};

pub struct LoggedInUser(pub User);

#[async_trait]
impl FromRequestParts<Service> for LoggedInUser {
    type Rejection = Error;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &Service,
    ) -> Result<Self, Self::Rejection> {
        let cookies = CookieJar::from_request_parts(parts, state).await?;

        let access_token = cookies
            .get("accessToken")
            .ok_or(Error::Unauthorized)?
            .value()
            .to_string();

        let user_id = decode_access_token(access_token)?;

        let user = state.get_user_by_id(user_id).await?;

        Ok(Self(user))
    }
}

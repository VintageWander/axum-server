use axum::extract::State;

use crate::{request::user::loggedin::LoggedInUser, service::Service, web::Web, WebResult};

pub async fn profile_user_handler(
    State(service): State<Service>,
    LoggedInUser(user): LoggedInUser,
) -> WebResult {
    Ok(Web::ok("Get user profile success", user.into_dto()))
}

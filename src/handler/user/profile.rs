use axum::extract::State;

use crate::{request::user::loggedin::LoggedInUser, service::Service, web::Web, WebResult};

pub async fn profile_user_handler(
    State(service): State<Service>,
    LoggedInUser(cookie_user): LoggedInUser,
) -> WebResult {
    Ok(Web::ok(
        "User profile get successfully",
        cookie_user.into_response(),
    ))
}

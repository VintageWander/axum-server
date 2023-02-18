use axum::extract::State;

use crate::{request::user::loggedin::LoggedInUser, web::Web, SharedState, WebResult};

pub async fn profile_user_handler(
    State(SharedState { user_service, .. }): State<SharedState>,
    LoggedInUser(cookie_user): LoggedInUser,
) -> WebResult {
    Ok(Web::ok(
        "User profile get successfully",
        cookie_user.into_response(),
    ))
}

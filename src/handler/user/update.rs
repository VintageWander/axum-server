use axum::extract::State;

use crate::{
    error::Error,
    extractors::param::ParamID,
    request::user::{loggedin::LoggedInUser, update::UpdateUserRequest},
    web::Web,
    SharedState, WebResult,
};

pub async fn update_user_handler(
    State(SharedState { user_service }): State<SharedState>,
    ParamID(user_id): ParamID,
    LoggedInUser(cookie_user): LoggedInUser,
    user_req: UpdateUserRequest,
) -> WebResult {
    if user_id != cookie_user.id {
        return Err(Error::Unauthorized);
    }

    let user = user_req.into_user(cookie_user)?;

    let user = user_service.update_user(user).await?;

    Ok(Web::ok("Update user successfully", user.into_response()))
}

use axum::extract::State;

use crate::{
    error::Error,
    extractors::param::ParamID,
    request::user::{loggedin::LoggedInUser, update::UpdateUserRequest},
    service::Service,
    web::Web,
    WebResult,
};

pub async fn update_user_handler(
    State(service): State<Service>,
    ParamID(user_id): ParamID,
    LoggedInUser(cookie_user): LoggedInUser,
    user_req: UpdateUserRequest,
) -> WebResult {
    if user_id != cookie_user.id {
        return Err(Error::Unauthorized);
    }

    let user = user_req.into_user(cookie_user)?;

    let user = service.update_user(user).await?;

    Ok(Web::ok("Update user successfully", user.into_response()))
}

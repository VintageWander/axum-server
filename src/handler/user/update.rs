use axum::extract::State;

use crate::{
    request::user::{loggedin::LoggedInUser, update::UpdateUserRequest},
    service::Service,
    web::Web,
    WebResult,
};

pub async fn update_user_handler(
    State(service): State<Service>,
    LoggedInUser(cookie_user): LoggedInUser,
    user_req: UpdateUserRequest,
) -> WebResult {
    let user = user_req.into_user(cookie_user)?;

    let user = service.update_user(user).await?;

    Web::ok("Update user successfully", user.into_response())
}

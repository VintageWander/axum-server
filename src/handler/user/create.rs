use axum::extract::State;

use crate::{request::user::create::CreateUserRequest, web::Web, SharedState, WebResult};

pub async fn create_user_handler(
    State(SharedState { user_service }): State<SharedState>,
    user_req: CreateUserRequest,
) -> WebResult {
    let new_user = user_service.create_user(user_req.try_into()?).await?;

    Ok(Web::ok(
        "Create user successfully",
        new_user.into_response(),
    ))
}

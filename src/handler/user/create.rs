use axum::extract::State;

use crate::{request::user::create::CreateUserRequest, services::Service, web::Web, WebResult};

pub async fn create_user_handler(
    State(service): State<Service>,
    user_req: CreateUserRequest,
) -> WebResult {
    let new_user = service.create_user(user_req.try_into()?).await?;

    Ok(Web::ok(
        "Create user successfully",
        new_user.into_response(),
    ))
}

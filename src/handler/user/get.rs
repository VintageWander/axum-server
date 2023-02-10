use axum::extract::State;

use crate::{web::Web, SharedState, WebResult};

pub async fn get_users_handler(
    State(SharedState { user_service }): State<SharedState>,
) -> WebResult {
    let users = user_service.get_users().await?;

    Ok(Web::ok("Get all users success", users))
}

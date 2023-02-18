use axum::extract::State;

use crate::{extractors::query_doc::UserQueryDocument, web::Web, SharedState, WebResult};

pub async fn get_users_handler(
    State(SharedState { user_service, .. }): State<SharedState>,
    UserQueryDocument(user_query): UserQueryDocument,
) -> WebResult {
    let users = user_service
        .get_users_by(user_query)
        .await?
        .map(|u| u.into_response())
        .collect::<Vec<_>>();

    Ok(Web::ok("Get all users success", users))
}

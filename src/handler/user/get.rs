use axum::extract::State;

use crate::{extractors::user_query::UserQueryDocument, services::Service, web::Web, WebResult};

pub async fn get_users_handler(
    State(service): State<Service>,
    UserQueryDocument(user_query): UserQueryDocument,
) -> WebResult {
    let users = service
        .get_users_by(user_query)
        .await?
        .into_iter()
        .map(|u| u.into_response())
        .collect::<Vec<_>>();

    Ok(Web::ok("Get all users success", users))
}

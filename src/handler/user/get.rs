use axum::extract::State;

use crate::{model::user::*, service::Service, web::Web, WebResult};

pub async fn get_users_handler(
    State(service): State<Service>,
    UserQuery(user_query): UserQuery,
) -> WebResult {
    let users = service
        .get_users_by(user_query)
        .await?
        .into_iter()
        .map(|u| u.into_response())
        .collect::<Vec<_>>();

    Ok(Web::ok("Get all users success", users))
}

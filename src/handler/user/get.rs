use axum::extract::State;

use crate::{
    extractors::param::ParamID, model::user::*, request::user::loggedin::LoggedInUser,
    service::Service, web::Web, WebResult,
};

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

pub async fn get_user_handler(
    State(service): State<Service>,
    user_or_guest: Option<LoggedInUser>,
    ParamID(user_id): ParamID,
) -> WebResult {
    let user = service.get_user_by_id(user_id).await?;

    Ok(Web::ok("Get user success", user.into_dto()))
}

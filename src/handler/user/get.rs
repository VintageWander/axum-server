use axum::extract::State;

use crate::{
    extractors::param::ParamID,
    model::{populated::user::UserPopulated, user::*},
    service::Service,
    web::Web,
    WebResult,
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
    ParamID(user_id): ParamID,
) -> WebResult {
    let user = service.get_user_by_id(user_id).await?;
    let files = service.get_files_by_owner(&user).await?;
    let shared_files = service
        .get_shared_files_from_accessor(&user)
        .await?;
    let folders = service.get_folders_by_owner(&user).await?;
    let shared_folders = service
        .get_shared_folders_from_accessor(&user)
        .await?;

    let result = UserPopulated::new(user, files, shared_files, folders, shared_folders);

    Ok(Web::ok("Get user success", result))
}

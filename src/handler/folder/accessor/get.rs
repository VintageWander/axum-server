use axum::extract::State;

use crate::{
    extractors::param::ParamID, request::user::loggedin::LoggedInUser, service::Service, web::Web,
    WebResult,
};

pub async fn get_folder_collaborators_handler(
    State(service): State<Service>,
    LoggedInUser(user): LoggedInUser,
    ParamID(folder_id): ParamID,
) -> WebResult {
    let folder = service
        .get_folder_by_id_owner(folder_id, &user)
        .await?;
    let accessors = service
        .get_accessors_from_shared_folder(&folder)
        .await?
        .into_iter()
        .map(|a| a.into_dto())
        .collect::<Vec<_>>();

    Ok(Web::ok("Get all accessors success", accessors))
}

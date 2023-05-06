use axum::extract::State;

use crate::{
    extractors::param::ParamID, request::user::loggedin::LoggedInUser, service::Service, web::Web,
    WebResult,
};

pub async fn get_folder_collaborator_handler(
    State(service): State<Service>,
    LoggedInUser(user): LoggedInUser,
    ParamID(folder_id): ParamID,
) -> WebResult {
    let folder = service
        .get_folder_by_id_owner(folder_id, &user)
        .await?;
    let collaborators = service
        .get_collaborators_from_shared_folder(&folder)
        .await?
        .into_iter()
        .map(|a| a.into_dto())
        .collect::<Vec<_>>();

    Web::ok("Get all collaborators success", collaborators)
}

use axum::extract::State;

use crate::{
    extractors::param::ParamID,
    request::{
        folder::collaborator::add::FolderAddCollaboratorRequest, user::loggedin::LoggedInUser,
    },
    service::Service,
    web::Web,
    WebResult,
};

pub async fn add_folder_collaborator_handler(
    State(service): State<Service>,
    LoggedInUser(user): LoggedInUser,
    ParamID(folder_id): ParamID,
    request: FolderAddCollaboratorRequest,
) -> WebResult {
    let folder = service
        .get_folder_by_id_owner(folder_id, &user)
        .await?;
    let collaborator = service.get_user_by_email(&request.email).await?;

    service
        .link_folder_collaborator(folder.id, collaborator.id)
        .await?;

    Ok(Web::ok("Added collaborator to folder success", ()))
}

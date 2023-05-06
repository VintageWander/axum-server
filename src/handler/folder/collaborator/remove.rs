use axum::extract::State;

use crate::{
    extractors::param::ParamID,
    request::{
        folder::collaborator::remove::FolderRemoveCollaboratorRequest, user::loggedin::LoggedInUser,
    },
    service::Service,
    web::Web,
    WebResult,
};

pub async fn remove_folder_collaborator_handler(
    State(service): State<Service>,
    LoggedInUser(user): LoggedInUser,
    ParamID(folder_id): ParamID,
    request: FolderRemoveCollaboratorRequest,
) -> WebResult {
    let folder = service
        .get_folder_by_id_owner(folder_id, &user)
        .await?;
    let collaborator = service.get_user_by_email(&request.email).await?;

    service
        .unlink_folder_collaborator(folder.id, collaborator.id)
        .await?;

    Web::ok("Removed collaborator to folder success", ())
}

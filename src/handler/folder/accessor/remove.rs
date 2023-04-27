use axum::extract::State;

use crate::{
    extractors::param::ParamID,
    request::{
        folder::accessor::remove::FolderRemoveAccessorRequest, user::loggedin::LoggedInUser,
    },
    service::Service,
    web::Web,
    WebResult,
};

pub async fn remove_folder_accessor_handler(
    State(service): State<Service>,
    LoggedInUser(user): LoggedInUser,
    ParamID(folder_id): ParamID,
    request: FolderRemoveAccessorRequest,
) -> WebResult {
    let folder = service
        .get_folder_by_id_owner(folder_id, &user)
        .await?;
    let accessor = service.get_user_by_email(&request.email).await?;

    service
        .unlink_folder_accessor(folder.id, accessor.id)
        .await?;

    Ok(Web::ok("Removed collaborator to folder success", ()))
}

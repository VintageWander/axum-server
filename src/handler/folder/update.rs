use axum::extract::State;

use crate::{
    extractors::param::ParamID,
    model::populated::folder::FolderPopulated,
    request::{folder::update::UpdateFolderRequest, user::loggedin::LoggedInUser},
    service::Service,
    web::Web,
    WebResult,
};

pub async fn update_folder_handler(
    State(service): State<Service>,
    ParamID(folder_id): ParamID,
    LoggedInUser(user): LoggedInUser,
    folder_req: UpdateFolderRequest,
) -> WebResult {
    let old_folder = match service
        .get_folder_by_id_owner(folder_id, &user)
        .await
        .ok()
    {
        Some(owned_folder) => owned_folder,
        None => {
            service
                .get_shared_folder_from_collaborator(folder_id, &user)
                .await?
        }
    };

    let folder_owner = service.get_user_by_id(old_folder.owner).await?;

    let target_folder = folder_req.into_folder(&folder_owner, old_folder)?;

    let updated_folder = service.update_folder(target_folder).await?;

    let result = FolderPopulated::new(updated_folder, user);

    Web::ok("Update a folder successfully", result)
}

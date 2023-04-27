use axum::extract::State;

use crate::{
    extractors::param::ParamID,
    request::{folder::update::UpdateFolderRequest, user::loggedin::LoggedInUser},
    service::Service,
    web::Web,
    WebResult,
};

pub async fn update_folder_handler(
    State(service): State<Service>,
    ParamID(folder_id): ParamID,
    LoggedInUser(cookie_user): LoggedInUser,
    folder_req: UpdateFolderRequest,
) -> WebResult {
    let old_folder = service
        .get_folder_by_id_owner(folder_id, &cookie_user)
        .await?;

    let target_folder = folder_req.into_folder_with_owner(old_folder, &cookie_user)?;

    let updated_folder = service
        .update_folder(target_folder)
        .await?
        .into_response();

    Ok(Web::ok("Update a folder successfully", updated_folder))
}

use axum::extract::State;
use tokio::spawn;

use crate::{
    extractors::param::ParamID, request::user::loggedin::LoggedInUser, web::Web, SharedState,
    WebResult,
};

pub async fn delete_folder_handler(
    State(SharedState { folder_service, .. }): State<SharedState>,
    ParamID(folder_id): ParamID,
    LoggedInUser(cookie_user): LoggedInUser,
) -> WebResult {
    let target_folder = folder_service
        .get_folder_by_id_owner(&folder_id, &cookie_user)
        .await?;

    // Spawn a thread to delete the folder
    // This code is async, that means the folder will be deleted in the background
    // while the web server returns result immediately
    spawn(async move { folder_service.delete_folder(target_folder).await });

    Ok(Web::ok("Delete folder successfully", ()))
}

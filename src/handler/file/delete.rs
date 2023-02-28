use axum::extract::State;
// use tokio::spawn;

use crate::{
    extractors::param::ParamID, request::user::loggedin::LoggedInUser, web::Web, SharedState,
    WebResult,
};

pub async fn delete_file_handler(
    State(SharedState { file_service, .. }): State<SharedState>,
    ParamID(file_id): ParamID,
    LoggedInUser(cookie_user): LoggedInUser,
) -> WebResult {
    let target_file = file_service
        .get_file_by_id_owner(file_id, &cookie_user)
        .await?;

    // Spawn a thread to delete the file
    // This code is async, that means the file will be deleted in the background
    // while the web server returns result immediately
    file_service.delete_file(target_file).await?;

    Ok(Web::ok("Delete file successfully", ()))
}

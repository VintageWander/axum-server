use axum::extract::State;

use crate::{
    request::{folder::create::CreateFolderRequest, user::loggedin::LoggedInUser},
    web::Web,
    SharedState, WebResult,
};

pub async fn create_folder_handler(
    State(SharedState { folder_service, .. }): State<SharedState>,
    LoggedInUser(cookie_user): LoggedInUser,
    folder_req: CreateFolderRequest,
) -> WebResult {
    let folder = folder_req.into_folder_with_owner(&cookie_user)?;

    let new_folder = folder_service
        .create_folder(folder)
        .await?
        .into_response();

    Ok(Web::ok("Create a new folder successfully", new_folder))
}

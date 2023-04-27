use axum::extract::State;

use crate::{
    model::populated::folder::FolderPopulated,
    request::{folder::create::CreateFolderRequest, user::loggedin::LoggedInUser},
    service::Service,
    web::Web,
    WebResult,
};

pub async fn create_folder_handler(
    State(service): State<Service>,
    LoggedInUser(cookie_user): LoggedInUser,
    folder_req: CreateFolderRequest,
) -> WebResult {
    let folder = folder_req.into_folder_with_owner(&cookie_user)?;

    let new_folder = service.create_folder(folder).await?;

    let result = FolderPopulated::new(new_folder, cookie_user);

    Ok(Web::ok("Create a new folder successfully", result))
}

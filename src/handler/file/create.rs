use axum::extract::State;

use crate::{
    request::{file::create::CreateFileRequest, user::loggedin::LoggedInUser},
    web::Web,
    SharedState, WebResult,
};

pub async fn create_file_handler(
    State(SharedState { file_service, .. }): State<SharedState>,
    LoggedInUser(cookie_user): LoggedInUser,
    file_req: CreateFileRequest,
) -> WebResult {
    let (file_model, bytes) = file_req.into_file(&cookie_user)?;

    let new_file = file_service
        .create_file(file_model, bytes)
        .await?
        .into_response();

    Ok(Web::ok("Upload file successfully", new_file))
}

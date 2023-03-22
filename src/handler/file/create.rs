use axum::extract::State;

use crate::{
    request::{file::create::CreateFileRequest, user::loggedin::LoggedInUser},
    services::Service,
    web::Web,
    WebResult,
};

pub async fn create_file_handler(
    State(service): State<Service>,
    LoggedInUser(cookie_user): LoggedInUser,
    file_req: CreateFileRequest,
) -> WebResult {
    let (file_model, bytes) = file_req.into_file(&cookie_user)?;

    let new_file = service
        .create_file(file_model, bytes)
        .await?
        .into_response();

    Ok(Web::ok("Upload file successfully", new_file))
}

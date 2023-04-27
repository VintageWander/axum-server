use axum::extract::State;

use crate::{
    model::populated::file::FilePopulated,
    request::{file::create::CreateFileRequest, user::loggedin::LoggedInUser},
    service::Service,
    web::Web,
    WebResult,
};

pub async fn create_file_handler(
    State(service): State<Service>,
    LoggedInUser(user): LoggedInUser,
    file_req: CreateFileRequest,
) -> WebResult {
    let (file_model, bytes) = file_req.into_file(&user)?;

    let new_file = service.create_file(file_model, bytes).await?;

    let result = FilePopulated::new(new_file, user);

    Ok(Web::ok("Upload file successfully", result))
}

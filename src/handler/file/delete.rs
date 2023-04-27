use axum::extract::State;
// use tokio::spawn;

use crate::{
    extractors::param::ParamID, request::user::loggedin::LoggedInUser, service::Service, web::Web,
    WebResult,
};

pub async fn delete_file_handler(
    State(service): State<Service>,
    ParamID(file_id): ParamID,
    LoggedInUser(cookie_user): LoggedInUser,
) -> WebResult {
    let target_file = service
        .get_file_by_id_owner(file_id, &cookie_user)
        .await?;

    // Spawn a thread to delete the file
    // This code is async, that means the file will be deleted in the background
    // while the web server returns result immediately
    service.delete_file(target_file).await?;

    Ok(Web::ok("Delete file successfully", ()))
}

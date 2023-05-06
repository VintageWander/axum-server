use axum::extract::State;
use tokio::spawn;
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
    let target_file = match service
        .get_file_by_id_owner(file_id, &cookie_user)
        .await
        .ok()
    {
        Some(owned_file) => owned_file,
        None => {
            service
                .get_shared_file_from_collaborator(file_id, &cookie_user)
                .await?
        }
    };

    // Spawn a thread to delete the file
    // This code is async, that means the file will be deleted in the background
    // while the web server returns result immediately
    spawn(async move { service.delete_file(target_file).await });

    Web::ok("Delete file successfully", ())
}

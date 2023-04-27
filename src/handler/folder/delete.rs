use axum::extract::State;
use tokio::spawn;

use crate::{
    extractors::param::ParamID, request::user::loggedin::LoggedInUser, service::Service, web::Web,
    WebResult,
};

pub async fn delete_folder_handler(
    State(service): State<Service>,
    ParamID(folder_id): ParamID,
    LoggedInUser(cookie_user): LoggedInUser,
) -> WebResult {
    let target_folder = match service
        .get_folder_by_id_owner(folder_id, &cookie_user)
        .await
        .ok()
    {
        Some(owned_folder) => owned_folder,
        None => {
            service
                .get_shared_folder_from_accessor(folder_id, &cookie_user)
                .await?
        }
    };

    // Spawn a thread to delete the folder
    // This code is async, that means the folder will be deleted in the background
    // while the web server returns result immediately
    spawn(async move { service.delete_folder(target_folder).await });

    Ok(Web::ok("Delete folder successfully", ()))
}

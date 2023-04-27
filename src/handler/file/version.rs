use axum::extract::State;

use crate::{
    extractors::param::ParamID, request::user::loggedin::LoggedInUser, service::Service, web::Web,
    WebResult,
};

pub async fn get_file_versions_handler(
    State(service): State<Service>,
    user_or_guest: Option<LoggedInUser>,
    ParamID(file_id): ParamID,
) -> WebResult {
    let file = if let Some(LoggedInUser(user)) = user_or_guest {
        service
            .get_file_by_id_owner(file_id, &user)
            .await?
    } else {
        service.get_public_file_by_id(file_id).await?
    };

    let versions: Vec<_> = service
        .get_file_versions(&file)
        .await?
        .into_iter()
        .map(|v| v.version_number)
        .collect();

    Ok(Web::ok("Get all versions of a file success", versions))
}

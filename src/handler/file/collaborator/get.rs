use axum::extract::State;

use crate::{
    extractors::param::ParamID, request::user::loggedin::LoggedInUser, service::Service, web::Web,
    WebResult,
};

pub async fn get_file_collaborators_handler(
    State(service): State<Service>,
    LoggedInUser(user): LoggedInUser,
    ParamID(file_id): ParamID,
) -> WebResult {
    let file = service
        .get_file_by_id_owner(file_id, &user)
        .await?;
    let collaborators = service
        .get_collaborators_from_shared_file(&file)
        .await?
        .into_iter()
        .map(|a| a.into_dto())
        .collect::<Vec<_>>();

    Ok(Web::ok("Get all collaborators success", collaborators))
}

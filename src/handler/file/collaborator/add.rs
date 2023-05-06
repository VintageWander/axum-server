use axum::extract::State;

use crate::{
    extractors::param::ParamID,
    request::{file::collaborator::add::FileAddCollaboratorRequest, user::loggedin::LoggedInUser},
    service::Service,
    web::Web,
    WebResult,
};

pub async fn add_file_collaborator_handler(
    State(service): State<Service>,
    LoggedInUser(user): LoggedInUser,
    ParamID(file_id): ParamID,
    request: FileAddCollaboratorRequest,
) -> WebResult {
    let file = service
        .get_file_by_id_owner(file_id, &user)
        .await?;
    let collaborator = service.get_user_by_email(&request.email).await?;

    service
        .link_file_collaborator(file.id, collaborator.id)
        .await?;

    Web::ok("Added collaborator to file success", ())
}

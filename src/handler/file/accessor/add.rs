use axum::extract::State;

use crate::{
    extractors::param::ParamID,
    request::{file::accessor::add::FileAddAccessorRequest, user::loggedin::LoggedInUser},
    service::Service,
    web::Web,
    WebResult,
};

pub async fn add_file_accessor_handler(
    State(service): State<Service>,
    LoggedInUser(user): LoggedInUser,
    ParamID(file_id): ParamID,
    request: FileAddAccessorRequest,
) -> WebResult {
    let file = service
        .get_file_by_id_owner(file_id, &user)
        .await?;
    let accessor = service.get_user_by_email(&request.email).await?;

    service
        .link_file_accessor(file.id, accessor.id)
        .await?;

    Ok(Web::ok("Added collaborator to file success", ()))
}

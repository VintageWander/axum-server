use axum::extract::State;

use crate::{
    extractors::param::ParamID,
    model::populated::file::FilePopulated,
    request::{file::update::UpdateFileRequest, user::loggedin::LoggedInUser},
    service::Service,
    web::Web,
    WebResult,
};

pub async fn update_file_handler(
    State(service): State<Service>,
    LoggedInUser(user): LoggedInUser,
    ParamID(file_id): ParamID,
    file_req: UpdateFileRequest,
) -> WebResult {
    // Find the old file
    let old_file = match service
        .get_file_by_id_owner(file_id, &user)
        .await
        .ok()
    {
        Some(owned_file) => owned_file,
        None => {
            service
                .get_shared_file_from_collaborator(file_id, &user)
                .await?
        }
    };
    let file_owner = service.get_user_by_id(old_file.owner).await?;

    let (target_file, bytes) = file_req.into_file(&file_owner, old_file)?;

    let updated_file = service.update_file(target_file, bytes).await?;

    let result = FilePopulated::new(updated_file, file_owner);

    Ok(Web::ok("Update file successfully", result))
}

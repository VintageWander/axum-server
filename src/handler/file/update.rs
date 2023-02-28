use axum::extract::State;

use crate::{
    extractors::param::ParamID,
    request::{file::update::UpdateFileRequest, user::loggedin::LoggedInUser},
    web::Web,
    SharedState, WebResult,
};

pub async fn update_file_handler(
    State(SharedState { file_service, .. }): State<SharedState>,
    LoggedInUser(cookie_user): LoggedInUser,
    ParamID(file_id): ParamID,
    file_req: UpdateFileRequest,
) -> WebResult {
    // Find the old file
    let old_file = file_service
        .get_file_by_id_owner(file_id, &cookie_user)
        .await?;

    let (target_file, bytes) = file_req.into_file(&cookie_user, old_file)?;

    let updated_file = file_service
        .update_file(target_file, bytes)
        .await?
        .into_response();

    Ok(Web::ok("Update file successfully", updated_file))
}

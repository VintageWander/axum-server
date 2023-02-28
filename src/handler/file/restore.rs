use std::str::FromStr;

use axum::extract::State;
use mongodb::bson::oid::ObjectId;

use crate::{
    request::{file::restore::FileRestoreRequest, user::loggedin::LoggedInUser},
    web::Web,
    SharedState, WebResult,
};

pub async fn restore_file_handler(
    State(SharedState { file_service, .. }): State<SharedState>,
    LoggedInUser(cookie_user): LoggedInUser,
    FileRestoreRequest {
        file_id,
        restore_version,
    }: FileRestoreRequest,
) -> WebResult {
    let file_id = ObjectId::from_str(&file_id)?;

    let requested_file = file_service
        .get_file_by_id_owner(file_id, &cookie_user)
        .await?;

    file_service
        .restore_file(&requested_file, restore_version)
        .await?;

    Ok(Web::ok(
        "Restore file success",
        requested_file.into_response(),
    ))
}

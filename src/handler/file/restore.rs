use std::str::FromStr;

use axum::extract::State;
use mongodb::bson::oid::ObjectId;

use crate::{
    request::{file::restore::FileRestoreRequest, user::loggedin::LoggedInUser},
    services::Service,
    web::Web,
    WebResult,
};

pub async fn restore_file_handler(
    State(service): State<Service>,
    LoggedInUser(cookie_user): LoggedInUser,
    FileRestoreRequest {
        file_id,
        restore_version,
    }: FileRestoreRequest,
) -> WebResult {
    let file_id = ObjectId::from_str(&file_id)?;

    let requested_file = service
        .get_file_by_id_owner(file_id, &cookie_user)
        .await?;

    service
        .restore_file(&requested_file, restore_version)
        .await?;

    Ok(Web::ok(
        "Restore file success",
        requested_file.into_response(),
    ))
}

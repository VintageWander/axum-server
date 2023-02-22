use axum::extract::State;

use crate::{request::file::create::CreateFileRequest, web::Web, SharedState, WebResult};

pub async fn create_file_handler(
    State(SharedState { .. }): State<SharedState>,
    file_req: CreateFileRequest,
) -> WebResult {
    dbg!(&file_req.position);
    dbg!(&file_req.visility);
    dbg!(&file_req.filename);

    Ok(Web::ok("Create file successfully", ()))
}

use std::io::Cursor;

use axum::{
    body::StreamBody,
    extract::{Query, State},
    http::header::{CONTENT_DISPOSITION, CONTENT_TYPE},
    response::{AppendHeaders, IntoResponse},
};
use mime_guess::from_path;
use serde::Deserialize;
use tokio_util::io::ReaderStream;

use crate::{
    extractors::param::ParamID, request::user::loggedin::LoggedInUser, SharedState, WebResult,
};

#[derive(Debug, Deserialize)]
pub struct VersionQuery {
    pub version: Option<i64>,
}

pub async fn file_content_handler(
    State(SharedState {
        storage,
        file_service,
        ..
    }): State<SharedState>,
    ParamID(file_id): ParamID,
    Query(VersionQuery {
        version: version_number_option,
    }): Query<VersionQuery>,
    user_or_guest: Option<LoggedInUser>,
) -> WebResult {
    dbg!(&version_number_option);
    let file = if let Some(LoggedInUser(user)) = user_or_guest {
        // If the user is logged in,
        // Get their file
        file_service
            .get_file_by_id_owner(file_id, &user)
            .await?
    } else {
        // Else,
        // Get the public file by id
        file_service
            .get_public_file_by_id(file_id)
            .await?
    };

    // Make the file path
    let file_path = if let Some(version_number) = version_number_option {
        format!(
            "{}/{}.{}",
            file.id,
            version_number,
            file.extension.to_string()
        )
    } else {
        format!("{}.{}", file.id, file.extension.to_string())
    };

    // Get the file from storage and write straight to the body without proxy storage
    let bytes = storage.get_object(file_path).await?;

    let cursor = Cursor::new(bytes);

    let stream = ReaderStream::new(cursor);

    let body = StreamBody::new(stream);

    // Move the full filename variable outside for better readability
    let full_filename = file.full_filename;

    // Guess the mime from the full filename
    let mime = from_path(&full_filename)
        .first_or_octet_stream()
        .to_string();

    Ok((
        AppendHeaders([
            (CONTENT_TYPE, mime),
            (
                CONTENT_DISPOSITION,
                format!("attachment; filename=\"{full_filename}\""),
            ),
        ]),
        body,
    )
        .into_response())
}

use axum::extract::{Query, State};

use crate::{
    extractors::file_query::FileQuery, request::user::loggedin::LoggedInUser, web::Web,
    SharedState, WebResult,
};

pub async fn get_files_handler(
    State(SharedState { file_service, .. }): State<SharedState>,
    user_or_guest: Option<LoggedInUser>,
    Query(file_query): Query<FileQuery>,
) -> WebResult {
    let mut all_files: Vec<_> = if let Some(LoggedInUser(user)) = user_or_guest {
        // If the user is logged in
        // Get all user's files
        let users_files = file_service
            .get_files_by_owner(&user)
            .await?
            .into_iter();

        // Get all public files
        let public_files = file_service
            .get_public_files()
            .await?
            .into_iter()
            .filter(|f| f.owner != user.id);

        users_files
            .chain(public_files)
            .map(|f| f.into_response())
            .collect()
    } else {
        // Else, for the guest,
        // Get all public files are there

        file_service
            .get_public_files()
            .await?
            .into_iter()
            .map(|f| f.into_response())
            .collect()
    };

    all_files.retain(|f| {
        let mut bool = true;
        if let Some(id) = &file_query.id {
            bool = bool && id == &f.id
        };
        if let Some(owner) = &file_query.owner {
            bool = bool && owner == &f.owner
        };
        if let Some(filename) = &file_query.filename {
            bool = bool && filename == &f.filename
        };
        if let Some(extension) = &file_query.extension {
            bool = bool && extension == &f.extension.to_string()
        };
        if let Some(full_filename) = &file_query.full_filename {
            bool = bool && full_filename == &f.full_filename
        };
        if let Some(visibility) = &file_query.visibility {
            bool = bool && visibility == &f.visibility.to_string()
        };
        if let Some(position) = &file_query.position {
            bool = bool && position == &f.position
        };
        if let Some(fullpath) = &file_query.fullpath {
            bool = bool && fullpath == &f.fullpath
        };
        if let Some(created_at) = &file_query.created_at {
            bool = bool && created_at == &f.created_at
        };
        if let Some(updated_at) = &file_query.updated_at {
            bool = bool && updated_at == &f.updated_at
        };
        bool
    });

    Ok(Web::ok("Get all files successful", all_files))
}

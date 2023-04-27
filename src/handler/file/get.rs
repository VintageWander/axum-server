use axum::extract::State;

use crate::{
    extractors::param::ParamID,
    model::{file::*, populated::file::FilePopulated},
    request::user::loggedin::LoggedInUser,
    service::Service,
    web::Web,
    WebResult,
};

pub async fn get_files_handler(
    State(service): State<Service>,
    user_or_guest: Option<LoggedInUser>,
    file_query: FileQueryFromRequest,
) -> WebResult {
    let mut all_files: Vec<_> = if let Some(LoggedInUser(user)) = user_or_guest {
        // If the user is logged in
        // Get all user's files
        let users_files = service
            .get_files_by_owner(&user)
            .await?
            .into_iter();

        // Get all files that the user was shared access to
        let shared_files = service
            .get_shared_files_from_accessor(&user)
            .await?;

        // Get all public files
        let public_files = service
            .get_public_files()
            .await?
            .into_iter()
            .filter(|f| f.owner != user.id);

        // Chain them together
        users_files
            .chain(shared_files)
            .chain(public_files)
            .map(|f| f.into_response())
            .collect()
    } else {
        // Else, for the guest,
        // Get all public files that are there

        service
            .get_public_files()
            .await?
            .into_iter()
            .map(|f| f.into_response())
            .collect()
    };

    // Retain is a function that filters the vec by a boolean function
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
            bool = bool && extension == &f.extension
        };
        if let Some(full_filename) = &file_query.full_filename {
            bool = bool && full_filename == &f.full_filename
        };
        if let Some(visibility) = &file_query.visibility {
            bool = bool && visibility == &f.visibility
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

// Get a file by id
pub async fn get_file_handler(
    State(service): State<Service>,
    user_or_guest: Option<LoggedInUser>,
    ParamID(file_id): ParamID,
) -> WebResult {
    // Checks if the requester is logged in or not
    let file = match user_or_guest {
        // Fetch the file by id and checks if he owns it
        Some(LoggedInUser(user)) => match service
            .get_file_by_id_owner(file_id, &user)
            .await
            .ok()
        {
            // If he does own it then return his file
            Some(owned_file) => owned_file,
            // Else, do another request to the database,
            // But this time checks if he is a collaborator
            None => {
                service
                    .get_shared_file_from_accessor(file_id, &user)
                    .await?
            }
        },
        // If the user is not logged in,
        // Return files in the public
        None => service.get_public_file_by_id(file_id).await?,
    };
    let file_owner = service.get_user_by_id(file.owner).await?;

    let result = FilePopulated::new(file, file_owner);

    Ok(Web::ok("Get file by id success", result))
}

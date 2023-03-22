use axum::extract::{Query, State};

use crate::{
    extractors::folder_query::FolderQuery, request::user::loggedin::LoggedInUser,
    services::Service, web::Web, WebResult,
};

pub async fn get_folders_handler(
    State(service): State<Service>,
    user_or_guest: Option<LoggedInUser>,
    Query(folder_query): Query<FolderQuery>,
) -> WebResult {
    let mut all_folders: Vec<_> = if let Some(LoggedInUser(cookie_user)) = user_or_guest {
        // If the user is logged in,

        // fetch all user's folders (including private)
        // filter out the root folder
        let users_folders = service
            .get_folders_by_owner(&cookie_user)
            .await?
            .into_iter()
            .filter(|f| f.folder_name != cookie_user.username)
            .map(|f| f.into_response());

        // fetch all public folders from everyone else (NOT including the user)
        let public_folders = service
            .get_public_folders()
            .await?
            .into_iter()
            .filter(|f| f.folder_name != cookie_user.username && f.owner != cookie_user.id)
            .map(|f| f.into_response());

        // chain 2 iterators and collect them as a vec

        users_folders.chain(public_folders).collect()
    } else {
        // If the user is not logged in

        // Just get the list of public folders
        service
            .get_public_folders()
            .await?
            .into_iter()
            .map(|f| f.into_response())
            .collect()
    };

    // Filter the collection by the query string

    all_folders.retain(|f| {
        let mut bool = true;
        if let Some(id) = &folder_query.id {
            bool = bool && &f.id == id
        }
        if let Some(owner) = &folder_query.owner {
            bool = bool && &f.owner == owner
        }
        if let Some(folder_name) = &folder_query.folder_name {
            bool = bool && &f.folder_name == folder_name
        }
        if let Some(position) = &folder_query.position {
            bool = bool && &f.position == position
        }
        if let Some(visibility) = &folder_query.visibility {
            bool = bool && &f.visibility == visibility
        }
        if let Some(fullpath) = &folder_query.fullpath {
            bool = bool && &f.fullpath == fullpath
        }
        if let Some(created_at) = &folder_query.created_at {
            bool = bool && &f.created_at == created_at
        }
        if let Some(updated_at) = &folder_query.updated_at {
            bool = bool && &f.updated_at == updated_at
        }
        bool
    });

    Ok(Web::ok("Get all folders successfully", all_folders))
}

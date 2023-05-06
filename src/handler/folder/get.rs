use std::str::FromStr;

use axum::extract::State;
use mongodb::bson::oid::ObjectId;

use crate::{
    extractors::param::ParamID,
    model::{folder::*, populated::folder::FolderPopulated},
    request::user::loggedin::LoggedInUser,
    service::Service,
    web::Web,
    WebResult,
};

pub async fn get_folders_handler(
    State(service): State<Service>,
    user_or_guest: Option<LoggedInUser>,
    folder_query: FolderQueryFromRequest,
) -> WebResult {
    let mut all_folders: Vec<_> = if let Some(LoggedInUser(cookie_user)) = user_or_guest {
        // If the user is logged in,

        // fetch all user's folders (including private)
        // filter out the root folder
        let users_folders = service
            .get_folders_by_owner(&cookie_user)
            .await?
            .into_iter()
            .filter(|f| f.folder_name != cookie_user.username); // filters root folder

        let shared_to_folders = service
            .get_shared_folders_from_collaborator(&cookie_user)
            .await?
            .into_iter();

        // fetch all public folders from everyone else (NOT including the user)
        let public_folders = service
            .get_public_folders()
            .await?
            .into_iter()
            .filter(|f| f.folder_name != cookie_user.username && f.owner != cookie_user.id);

        // chain 2 iterators and collect them as a vec

        users_folders
            .chain(shared_to_folders)
            .chain(public_folders)
            .map(|f| f.into_response())
            .collect()
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

    let mut result = vec![];
    for folder in all_folders {
        let folder_owner = service
            .get_user_by_id(ObjectId::from_str(&folder.owner)?)
            .await?;
        let populated = FolderPopulated {
            folder,
            owner: folder_owner.into_dto(),
        };
        result.push(populated);
    }

    Web::ok("Get all folders successfully", result)
}

// Get a folder by id
pub async fn get_folder_handler(
    State(service): State<Service>,
    user_or_guest: Option<LoggedInUser>,
    ParamID(folder_id): ParamID,
) -> WebResult {
    // Checks if the requester is logged in or not
    let folder = match user_or_guest {
        // Fetch the folder by id and checks if he owns it
        Some(LoggedInUser(user)) => match service
            .get_folder_by_id_owner(folder_id, &user)
            .await
            .ok()
        {
            // If he does own it then return his folder
            Some(owned_folder) => owned_folder,
            // Else, do another request to the database,
            // But this time checks if he is a collaborator
            None => {
                match service
                    .get_shared_folder_from_collaborator(folder_id, &user)
                    .await
                    .ok()
                {
                    Some(shared_folder) => shared_folder,
                    None => service.get_public_folder_by_id(folder_id).await?,
                }
            }
        },
        // If the user is not logged in,
        // Return folders in the public
        None => service.get_public_folder_by_id(folder_id).await?,
    };
    let folder_owner = service.get_user_by_id(folder.owner).await?;

    let result = FolderPopulated::new(folder, folder_owner);

    Web::ok("Get folder by id success", result)
}

use axum::extract::State;

use crate::{
    extractors::param::ParamID,
    model::{populated::user::UserPopulated, user::*},
    request::user::loggedin::LoggedInUser,
    service::Service,
    web::Web,
    WebResult,
};

pub async fn get_users_handler(
    State(service): State<Service>,
    UserQuery(user_query): UserQuery,
) -> WebResult {
    let users = service
        .get_users_by(user_query)
        .await?
        .into_iter()
        .map(|u| u.into_response())
        .collect::<Vec<_>>();

    Ok(Web::ok("Get all users success", users))
}

pub async fn get_user_handler(
    State(service): State<Service>,
    user_or_guest: Option<LoggedInUser>,
    ParamID(user_id): ParamID,
) -> WebResult {
    // The user struct
    let user = match &user_or_guest {
        Some(LoggedInUser(user)) => user.clone(),
        None => service.get_user_by_id(user_id).await?.clone(),
    };

    // The user's public files
    let public_files = service.get_public_files_by_owner(&user).await?;

    // The user's shared files, owned by them
    let my_shared_files = match &user_or_guest {
        Some(LoggedInUser(user)) => service.get_shared_files_by_owner(user).await?,
        None => vec![],
    };

    let other_shared_files = match &user_or_guest {
        Some(LoggedInUser(user)) => {
            service
                .get_shared_files_from_accessor(user)
                .await?
        }
        None => vec![],
    };

    let private_files = match &user_or_guest {
        Some(LoggedInUser(user)) => service.get_private_files_by_owner(user).await?,
        None => vec![],
    };

    let public_folders = service.get_public_folders_by_owner(&user).await?;

    let my_shared_folders = match &user_or_guest {
        Some(LoggedInUser(user)) => service.get_shared_folders_by_owner(user).await?,
        None => vec![],
    };

    let other_shared_folders = match &user_or_guest {
        Some(LoggedInUser(user)) => {
            service
                .get_shared_folders_from_accessor(user)
                .await?
        }
        None => vec![],
    };

    let private_folders = match &user_or_guest {
        Some(LoggedInUser(user)) => service
            .get_private_folders_by_owner(user)
            .await?
            .into_iter()
            .filter(|f| {
                (f.folder_name != format!("{}/", user.username))
                    && (f.fullpath != format!("{}/", user.username))
            })
            .collect(),
        None => vec![],
    };

    let result = UserPopulated::new(
        user,
        public_files,
        my_shared_files,
        other_shared_files,
        private_files,
        public_folders,
        my_shared_folders,
        other_shared_folders,
        private_folders,
    );

    Ok(Web::ok("Get user success", result))
}

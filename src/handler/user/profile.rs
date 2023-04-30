use axum::extract::State;

use crate::{
    model::populated::user::UserPopulated, request::user::loggedin::LoggedInUser, service::Service,
    web::Web, WebResult,
};

pub async fn profile_user_handler(
    State(service): State<Service>,
    LoggedInUser(user): LoggedInUser,
) -> WebResult {
    // The user's public files
    let public_files = service.get_public_files_by_owner(&user).await?;

    // The user's shared files, owned by them
    let my_shared_files = service.get_shared_files_by_owner(&user).await?;

    let other_shared_files = service
        .get_shared_files_from_collaborator(&user)
        .await?;

    let private_files = service.get_private_files_by_owner(&user).await?;

    let public_folders = service.get_public_folders_by_owner(&user).await?;

    let my_shared_folders = service.get_shared_folders_by_owner(&user).await?;

    let other_shared_folders = service
        .get_shared_folders_from_collaborator(&user)
        .await?;

    let private_folders = service
        .get_private_folders_by_owner(&user)
        .await?
        .into_iter()
        .filter(|f| {
            (f.folder_name != format!("{}/", user.username))
                && (f.fullpath != format!("{}/", user.username))
        })
        .collect();

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

    Ok(Web::ok("Get user profile success", result))
}

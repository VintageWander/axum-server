use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    handler::folder::{
        create::create_folder_handler, delete::delete_folder_handler, get::get_folders_handler,
        update::update_folder_handler,
    },
    SharedState,
};

use super::user::delete_user_route;

pub fn folders_route() -> Router<SharedState> {
    Router::new()
        .merge(get_folders_route())
        .merge(create_folder_route())
        .merge(update_folder_route())
        .merge(delete_user_route())
}

pub fn get_folders_route() -> Router<SharedState> {
    Router::new().route("/", get(get_folders_handler))
}

pub fn create_folder_route() -> Router<SharedState> {
    Router::new().route("/", post(create_folder_handler))
}

pub fn update_folder_route() -> Router<SharedState> {
    Router::new().route("/:folder_id", put(update_folder_handler))
}

pub fn delete_folder_route() -> Router<SharedState> {
    Router::new().route("/:folder_id", delete(delete_folder_handler))
}

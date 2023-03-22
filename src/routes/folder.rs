use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    handler::folder::{
        create::create_folder_handler, delete::delete_folder_handler, get::get_folders_handler,
        update::update_folder_handler,
    },
    services::Service,
};

pub fn folders_route() -> Router<Service> {
    Router::new().nest(
        "/folders",
        Router::new()
            .merge(get_folders_route())
            .merge(create_folder_route())
            .merge(update_folder_route())
            .merge(delete_folder_route()),
    )
}

pub fn get_folders_route() -> Router<Service> {
    Router::new().route("/", get(get_folders_handler))
}

pub fn create_folder_route() -> Router<Service> {
    Router::new().route("/", post(create_folder_handler))
}

pub fn update_folder_route() -> Router<Service> {
    Router::new().route("/:folder_id", put(update_folder_handler))
}

pub fn delete_folder_route() -> Router<Service> {
    Router::new().route("/:folder_id", delete(delete_folder_handler))
}

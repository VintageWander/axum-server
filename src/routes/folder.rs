use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    handler::folder::{
        collaborator::{
            add::add_folder_collaborator_handler, get::get_folder_collaborator_handler,
            remove::remove_folder_collaborator_handler,
        },
        create::create_folder_handler,
        delete::delete_folder_handler,
        get::{get_folder_handler, get_folders_handler},
        update::update_folder_handler,
    },
    service::Service,
};

pub fn folders_route() -> Router<Service> {
    Router::new().nest(
        "/folder",
        Router::new()
            .merge(get_folders_route())
            .merge(get_folder_route())
            .merge(create_folder_route())
            .merge(update_folder_route())
            .merge(delete_folder_route())
            .merge(get_folder_collaborator_route())
            .merge(add_folder_collaborator_route())
            .merge(remove_folder_collaborator_route()),
    )
}

pub fn get_folders_route() -> Router<Service> {
    Router::new().route("/", get(get_folders_handler))
}

pub fn get_folder_route() -> Router<Service> {
    Router::new().route("/:folder_id", get(get_folder_handler))
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

pub fn get_folder_collaborator_route() -> Router<Service> {
    Router::new().route(
        "/:folder_id/collaborator",
        get(get_folder_collaborator_handler),
    )
}

pub fn add_folder_collaborator_route() -> Router<Service> {
    Router::new().route(
        "/:folder_id/collaborator",
        post(add_folder_collaborator_handler),
    )
}

pub fn remove_folder_collaborator_route() -> Router<Service> {
    Router::new().route(
        "/:folder_id/collaborator",
        delete(remove_folder_collaborator_handler),
    )
}

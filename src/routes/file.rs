use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    handler::file::{
        collaborator::{
            add::add_file_collaborator_handler, get::get_file_collaborators_handler,
            remove::remove_file_collaborator_handler,
        },
        content::file_content_handler,
        create::create_file_handler,
        delete::delete_file_handler,
        get::{get_file_handler, get_files_handler},
        restore::restore_file_handler,
        update::update_file_handler,
        version::get_file_versions_handler,
    },
    service::Service,
};

pub fn file_routes() -> Router<Service> {
    Router::new().nest(
        "/files",
        Router::new()
            .merge(get_files_route())
            .merge(get_file_route())
            .merge(get_file_versions_route())
            .merge(file_content_route())
            .merge(create_file_route())
            .merge(update_file_route())
            .merge(restore_file_route())
            .merge(delete_file_route())
            .merge(get_file_collaborators_route())
            .merge(add_file_collaborator_route())
            .merge(remove_file_collaborator_route()),
    )
}

pub fn get_files_route() -> Router<Service> {
    Router::new().route("/", get(get_files_handler))
}

pub fn get_file_route() -> Router<Service> {
    Router::new().route("/:file_id", get(get_file_handler))
}

pub fn file_content_route() -> Router<Service> {
    Router::new().route("/:file_id", get(file_content_handler))
}

pub fn get_file_versions_route() -> Router<Service> {
    Router::new().route("/versions/:file_id", get(get_file_versions_handler))
}

pub fn create_file_route() -> Router<Service> {
    Router::new().route("/", post(create_file_handler))
}

pub fn update_file_route() -> Router<Service> {
    Router::new().route("/:file_id", put(update_file_handler))
}

pub fn restore_file_route() -> Router<Service> {
    Router::new().route("/restore", put(restore_file_handler))
}

pub fn delete_file_route() -> Router<Service> {
    Router::new().route("/:file_id", delete(delete_file_handler))
}

pub fn get_file_collaborators_route() -> Router<Service> {
    Router::new().route(
        "/:file_id/collaborator",
        get(get_file_collaborators_handler),
    )
}

pub fn add_file_collaborator_route() -> Router<Service> {
    Router::new().route(
        "/:file_id/collaborator",
        post(add_file_collaborator_handler),
    )
}

pub fn remove_file_collaborator_route() -> Router<Service> {
    Router::new().route(
        "/:file_id/collaborator",
        delete(remove_file_collaborator_handler),
    )
}

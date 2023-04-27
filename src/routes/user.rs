use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    handler::user::{
        create::create_user_handler, delete::delete_user_handler, get::get_users_handler,
        profile::profile_user_handler, update::update_user_handler,
    },
    service::Service,
};

pub fn user_routes() -> Router<Service> {
    Router::new().nest(
        "/user",
        Router::new()
            .merge(get_users_route())
            .merge(create_users_route())
            .merge(profile_user_route())
            .merge(update_user_route())
            .merge(delete_user_route()),
    )
}

pub fn get_users_route() -> Router<Service> {
    Router::new().route("/", get(get_users_handler))
}

pub fn create_users_route() -> Router<Service> {
    Router::new().route("/", post(create_user_handler))
}

pub fn profile_user_route() -> Router<Service> {
    Router::new().route("/profile", get(profile_user_handler))
}

pub fn update_user_route() -> Router<Service> {
    Router::new().route("/", put(update_user_handler))
}

pub fn delete_user_route() -> Router<Service> {
    Router::new().route("/", delete(delete_user_handler))
}

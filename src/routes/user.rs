use axum::{routing::get, Router};

use crate::{handler::user::get::get_users_handler, SharedState};

pub fn user_routes() -> Router<SharedState> {
    Router::new().nest("/user", get_users_route())
}

pub fn get_users_route() -> Router<SharedState> {
    Router::new().route("/", get(get_users_handler))
}

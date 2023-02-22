use axum::{routing::post, Router};

use crate::{handler::file::create::create_file_handler, SharedState};

pub fn file_routes() -> Router<SharedState> {
    Router::new().nest("/files", Router::new().merge(create_file_route()))
}

pub fn create_file_route() -> Router<SharedState> {
    Router::new().route("/", post(create_file_handler))
}

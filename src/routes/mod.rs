use axum::{
    extract::{Path, State},
    routing::options,
    Router,
};

use crate::{service::Service, web::Web, WebResult};

use self::{auth::auth_routes, file::file_routes, folder::folders_route, user::user_routes};

pub mod auth;
pub mod file;
pub mod folder;
pub mod user;

pub async fn ok_handler(State(_): State<Service>, Path(rest): Path<String>) -> WebResult {
    Ok(Web::ok("OK", ()))
}

pub fn app_routes() -> Router<Service> {
    Router::new()
        .route("/*rest", options(ok_handler))
        .merge(auth_routes())
        .merge(user_routes())
        .merge(folders_route())
        .merge(file_routes())
}

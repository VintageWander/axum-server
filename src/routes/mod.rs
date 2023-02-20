use axum::Router;

use crate::SharedState;

use self::{auth::auth_routes, folder::folders_route, user::user_routes};

pub mod auth;
pub mod folder;
pub mod user;

pub fn app_routes() -> Router<SharedState> {
    Router::new()
        .merge(auth_routes())
        .merge(user_routes())
        .merge(folders_route())
}

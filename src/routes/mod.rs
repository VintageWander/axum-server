use axum::Router;

use crate::SharedState;

use self::{auth::auth_routes, user::user_routes};

pub mod auth;
pub mod user;

pub fn app_routes() -> Router<SharedState> {
    Router::new()
        .merge(auth_routes())
        .merge(user_routes())
}

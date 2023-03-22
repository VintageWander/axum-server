use axum::{
    routing::{delete, post},
    Router,
};

use crate::{
    handler::auth::{login::login_handler, logout::logout_handler, refresh::refresh_handler},
    services::Service,
};

pub fn auth_routes() -> Router<Service> {
    Router::new().nest(
        "/auth",
        Router::new()
            .merge(login_route())
            .merge(refresh_route())
            .merge(logout_route()),
    )
}

pub fn login_route() -> Router<Service> {
    Router::new().route("/login", post(login_handler))
}

pub fn refresh_route() -> Router<Service> {
    Router::new().route("/refresh", post(refresh_handler))
}

pub fn logout_route() -> Router<Service> {
    Router::new().route("/logout", delete(logout_handler))
}

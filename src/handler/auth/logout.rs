use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    helper::auth::{
        cookie::{make_access_cookie, make_refresh_cookie},
        encode::{make_access_token, make_refresh_token},
    },
    model::user::User,
    request::user::loggedin::LoggedInUser,
    services::Service,
    web::Web,
    WebResult,
};

pub async fn logout_handler(
    State(service): State<Service>,
    cookies: CookieJar,
    LoggedInUser(cookie_user): LoggedInUser,
) -> WebResult {
    // Create access and refresh tokens
    let (access_token, refresh_token) = (
        make_access_token(&cookie_user)?,
        make_refresh_token(&cookie_user)?,
    );

    // Create access and refresh cookies
    let (access_cookie, refresh_cookie) = (
        make_access_cookie(access_token),
        make_refresh_cookie(refresh_token),
    );

    // Create a user clone to set the refresh token to none
    let user = User {
        refresh_token: "".into(),
        ..cookie_user
    };

    service.update_user(user).await?;

    Ok((
        StatusCode::OK,
        cookies
            .remove(access_cookie)
            .remove(refresh_cookie),
        Web::ok("Logout success", ()),
    )
        .into_response())
}

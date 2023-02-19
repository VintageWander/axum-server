use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    helper::auth::{
        cookie::{make_access_cookie, make_refresh_cookie},
        encode::{make_access_token, make_refresh_token},
    },
    request::user::loggedin::LoggedInUser,
    web::Web,
    SharedState, WebResult,
};

pub async fn logout_handler(
    State(SharedState { user_service, .. }): State<SharedState>,
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

    Ok((
        StatusCode::OK,
        cookies
            .remove(access_cookie)
            .remove(refresh_cookie),
        Web::ok("Logout success", ()),
    )
        .into_response())
}

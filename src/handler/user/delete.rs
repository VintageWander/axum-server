use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;
use tokio::spawn;

use crate::{
    error::Error,
    extractors::param::ParamID,
    helper::auth::{
        cookie::{make_access_cookie, make_refresh_cookie},
        encode::{make_access_token, make_refresh_token},
    },
    request::user::{delete::DeleteUserRequest, loggedin::LoggedInUser},
    web::Web,
    SharedState, WebResult,
};

pub async fn delete_user_handler(
    State(SharedState { user_service, .. }): State<SharedState>,
    cookies: CookieJar,
    ParamID(user_id): ParamID,
    LoggedInUser(cookie_user): LoggedInUser,
    user_req: DeleteUserRequest,
) -> WebResult {
    // Check if the param id and the logged in user id is valid
    if user_id != cookie_user.id {
        return Err(Error::Unauthorized);
    }

    // Check if the user request form password matches with the user's password
    if user_req.password != cookie_user.password {
        return Err(Error::InvalidPassword);
    }

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

    // Delete the user
    spawn(async move { user_service.delete_user(cookie_user).await });

    // Remove the cookies, to indicate a logout
    Ok((
        StatusCode::OK,
        cookies
            .remove(access_cookie)
            .remove(refresh_cookie),
        Web::ok("Deleted user successfully", ()),
    )
        .into_response())
}

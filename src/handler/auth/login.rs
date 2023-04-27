use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    helper::auth::{
        cookie::{make_access_cookie, make_refresh_cookie},
        encode::{make_access_token, make_refresh_token},
    },
    model::user::User,
    request::user::login::LoginUserRequest,
    service::Service,
    web::Web,
    WebResult,
};

pub async fn login_handler(
    State(service): State<Service>,
    cookies: CookieJar,
    user_req: LoginUserRequest,
) -> WebResult {
    let user = service
        .get_user_by_login_info(&user_req.username, &user_req.password)
        .await?;

    let (access_token, refresh_token) = (make_access_token(&user)?, make_refresh_token(&user)?);

    let (access_cookie, refresh_cookie) = (
        make_access_cookie(access_token),
        make_refresh_cookie(refresh_token.clone()),
    );

    let user = User {
        refresh_token,
        ..user
    };

    let update_user = service.update_user(user).await?;

    Ok((
        StatusCode::OK,
        cookies.add(access_cookie).add(refresh_cookie),
        Web::ok("Login success", update_user.into_response()),
    )
        .into_response())
}

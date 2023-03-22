use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    error::Error,
    helper::auth::{
        cookie::make_refresh_cookie, decode::decode_refresh_token, encode::make_refresh_token,
    },
    services::Service,
    web::Web,
    WebResult,
};

pub async fn refresh_handler(State(service): State<Service>, cookies: CookieJar) -> WebResult {
    let refresh_token = cookies
        .get("refreshToken")
        .ok_or(Error::Unauthorized)?
        .value()
        .to_string();

    let user_id = decode_refresh_token(refresh_token.clone())?;

    let user = service.get_user_by_id(user_id).await?;

    if user.refresh_token != refresh_token {
        return Err(Error::Unauthorized);
    }

    let new_refresh_token = make_refresh_token(&user)?;
    let new_refresh_cookie = make_refresh_cookie(new_refresh_token);

    Ok((
        StatusCode::OK,
        cookies.add(new_refresh_cookie),
        Web::ok("Token successfully refreshed", ()),
    )
        .into_response())
}

use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    error::Error,
    helper::auth::{
        cookie::make_access_cookie, decode::decode_refresh_token, encode::make_access_token,
    },
    service::Service,
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

    let new_access_token = make_access_token(&user)?;
    let new_access_cookie = make_access_cookie(new_access_token);

    Ok((
        StatusCode::OK,
        cookies.add(new_access_cookie),
        Web::ok("Token successfully refreshed", ()),
    )
        .into_response())
}

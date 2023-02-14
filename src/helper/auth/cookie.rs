use axum_extra::extract::cookie::{Cookie, SameSite};
use cookie::time::Duration;

pub fn make_access_cookie<'a>(access_jwt: String) -> Cookie<'a> {
    Cookie::build("accessToken", access_jwt)
        .path("/")
        .max_age(Duration::hours(1))
        .http_only(true)
        .same_site(SameSite::None)
        .finish()
}

pub fn make_refresh_cookie<'a>(refresh_jwt: String) -> Cookie<'a> {
    Cookie::build("refreshToken", refresh_jwt)
        .path("/")
        .max_age(Duration::hours(3))
        .http_only(true)
        .same_site(SameSite::None)
        .finish()
}

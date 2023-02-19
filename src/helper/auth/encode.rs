use chrono::{Duration, Utc};
use dotenv::var;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use crate::{model::user::User, Result};

use super::{Claims, JwtType};

pub fn encode_jwt(user: &User, token_type: JwtType) -> Result<String> {
    let (jwt_secret, duration) = match token_type {
        JwtType::Access => (var("JWT_ACCESS").unwrap(), Duration::hours(1)),
        JwtType::Refresh => (var("JWT_REFRESH").unwrap(), Duration::hours(3)),
    };

    let expiration = Utc::now()
        .checked_add_signed(duration)
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        name: user.username.clone(),
        exp: expiration,
    };

    let header = Header::new(Algorithm::HS512);

    let key = EncodingKey::from_secret(jwt_secret.as_bytes());

    let encode = encode(&header, &claims, &key)?;

    Ok(encode)
}

pub fn make_access_token(user: &User) -> Result<String> {
    encode_jwt(user, JwtType::Access)
}

pub fn make_refresh_token(user: &User) -> Result<String> {
    encode_jwt(user, JwtType::Refresh)
}

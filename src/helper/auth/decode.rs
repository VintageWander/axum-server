use std::str::FromStr;

use dotenv::var;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use mongodb::bson::oid::ObjectId;

use crate::Result;

use super::{Claims, JwtType};

pub fn decode_jwt(jwt: String, token_type: JwtType) -> Result<ObjectId> {
    let jwt_secret = match token_type {
        JwtType::Access => var("JWT_ACCESS").unwrap(),
        JwtType::Refresh => var("JWT_REFRESH").unwrap(),
    };

    let key = DecodingKey::from_secret(jwt_secret.as_bytes());

    let validation = Validation::new(Algorithm::HS512);

    let decoded = decode::<Claims>(&jwt, &key, &validation)?;

    Ok(ObjectId::from_str(&decoded.claims.sub)?)
}

pub fn decode_access_token(jwt: String) -> Result<ObjectId> {
    decode_jwt(jwt, JwtType::Access)
}

pub fn decode_refresh_token(jwt: String) -> Result<ObjectId> {
    decode_jwt(jwt, JwtType::Refresh)
}

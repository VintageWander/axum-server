use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub mod cookie;
pub mod decode;
pub mod encode;

pub enum JwtType {
    Access,
    Refresh,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: ObjectId,
    name: String,
    exp: usize,
}

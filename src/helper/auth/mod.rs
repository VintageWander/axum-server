use serde::{Deserialize, Serialize};

pub mod cookie;
pub mod decode;
pub mod encode;

pub enum JwtType {
    Access,
    Refresh,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    name: String,
    exp: usize,
}

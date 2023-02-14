use serde::{Deserialize, Serialize};

use crate::model::user::User;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    #[serde(rename = "_id")]
    pub id: String,

    pub username: String,
    pub email: String,

    pub created_at: i64,
    pub updated_at: i64,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        UserResponse {
            id: u.id.to_string(),
            username: u.username,
            email: u.email,
            created_at: u.created_at,
            updated_at: u.updated_at,
        }
    }
}

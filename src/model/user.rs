use backend_macros::Dto;

use mongodb::bson::{doc, oid::ObjectId};
use mongoose::Model;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::validation::user::{check_password, check_username};

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Model, Dto)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,

    #[validate(custom = "check_username")]
    pub username: String,

    #[validate(email(message = "Email is invalid"))]
    pub email: String,

    #[validate(custom = "check_password")]
    pub password: String,

    pub refresh_token: String,

    pub created_at: i64,
    pub updated_at: i64,
}

impl User {
    pub fn into_response(self) -> UserDTO {
        self.into_dto()
    }
}

use chrono::Utc;
use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    error::Error,
    response::user::UserResponse,
    validation::user::{check_password, check_username},
    Result,
};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
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

impl From<User> for Document {
    fn from(u: User) -> Self {
        doc! {
            "username": u.username,
            "email": u.email,
            "password": u.password,
            "refreshToken": u.refresh_token,
            "createdAt": u.created_at,
            "updatedAt": u.updated_at,
        }
    }
}

impl User {
    pub fn builder() -> UserBuilder {
        UserBuilder::new()
    }

    pub fn into_response(self) -> UserResponse {
        self.into()
    }
}

#[derive(Default)]
pub struct UserBuilder {
    pub id: Option<ObjectId>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub refresh_token: Option<String>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

impl UserBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn id(mut self, id: ObjectId) -> Self {
        self.id = Some(id);
        self
    }
    pub fn username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }
    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
    pub fn password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }
    pub fn refresh_token(mut self, refresh_token: String) -> Self {
        self.refresh_token = Some(refresh_token);
        self
    }
    pub fn created_at(mut self, created_at: i64) -> Self {
        self.created_at = Some(created_at);
        self
    }
    pub fn updated_at(mut self, updated_at: i64) -> Self {
        self.updated_at = Some(updated_at);
        self
    }
    pub fn build(self) -> Result<User> {
        let user = User {
            id: self.id.unwrap_or(ObjectId::new()),
            username: self.username.unwrap_or_default(),
            email: self.email.unwrap_or_default(),
            password: self.password.unwrap_or_default(),
            refresh_token: self.refresh_token.unwrap_or_default(),
            created_at: self
                .created_at
                .unwrap_or(Utc::now().timestamp_millis()),
            updated_at: self
                .updated_at
                .unwrap_or(Utc::now().timestamp_millis()),
        };

        user.validate()?;

        Ok(user)
    }
}

impl From<User> for UserBuilder {
    fn from(u: User) -> Self {
        Self {
            id: Some(u.id),
            username: Some(u.username),
            password: Some(u.password),
            email: Some(u.email),
            refresh_token: Some(u.refresh_token),
            created_at: Some(u.created_at),
            updated_at: Some(u.updated_at),
        }
    }
}

impl TryFrom<UserBuilder> for User {
    type Error = Error;
    fn try_from(builder: UserBuilder) -> std::result::Result<Self, Self::Error> {
        builder.build()
    }
}

use std::vec::IntoIter;

use mongodb::bson::{oid::ObjectId, Document};

use crate::{model::user::User, Result};

use super::UserService;

impl UserService {
    pub async fn get_users(&self) -> Result<IntoIter<User>> {
        self.user_repo.get_users().await
    }

    pub async fn get_users_by(&self, doc: Document) -> Result<IntoIter<User>> {
        self.user_repo.get_users_by(doc).await
    }

    pub async fn get_user_by_id(&self, user_id: &ObjectId) -> Result<User> {
        self.user_repo.get_user_by_id(user_id).await
    }

    pub async fn get_user_by_login_info(&self, username: &str, password: &str) -> Result<User> {
        self.user_repo
            .get_user_by_login_info(username, password)
            .await
    }
}

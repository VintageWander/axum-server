use mongodb::bson::oid::ObjectId;

use crate::{model::user::User, Result};

use super::UserService;

impl UserService {
    pub async fn get_users(&self) -> Result<Vec<User>> {
        self.user_repo.get_users().await
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

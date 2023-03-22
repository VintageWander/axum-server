use mongodb::bson::oid::ObjectId;
use mongodb::bson::Document;

use crate::model::user::User;
use crate::services::Service;
use crate::Result;

impl Service {
    pub async fn get_users(&self) -> Result<Vec<User>> {
        self.user_repo.get_users().await
    }

    pub async fn get_users_by(&self, doc: Document) -> Result<Vec<User>> {
        self.user_repo.get_users_by(doc).await
    }

    pub async fn get_user_by_id(&self, user_id: ObjectId) -> Result<User> {
        self.user_repo.get_user_by_id(user_id).await
    }

    pub async fn get_user_by_login_info(&self, username: &str, password: &str) -> Result<User> {
        self.user_repo
            .get_user_by_login_info(username, password)
            .await
    }
}

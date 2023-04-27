use mongodb::bson::oid::ObjectId;
use mongodb::bson::Document;

use crate::model::user::User;
use crate::service::Service;
use crate::Result;

impl Service {
    pub async fn get_users(&self) -> Result<Vec<User>> {
        self.user.get_many(User::blank()).await
    }

    pub async fn get_users_by(&self, doc: Document) -> Result<Vec<User>> {
        self.user.get_many(doc).await
    }

    pub async fn get_user_by_id(&self, user_id: ObjectId) -> Result<User> {
        self.user.get_one(User::id(user_id)).await
    }

    pub async fn get_user_by_login_info(&self, username: &str, password: &str) -> Result<User> {
        self.user
            .get_one(User::username(username).password(password))
            .await
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User> {
        self.user.get_one(User::email(email)).await
    }
}

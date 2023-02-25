use mongodb::bson::{doc, oid::ObjectId, Document};

use super::UserRepo;
use crate::{model::user::User, Result};

impl UserRepo {
    pub async fn get_users_by(&self, doc: Document) -> Result<Vec<User>> {
        self.user_dao.get_multiple(doc).await
    }

    pub async fn get_users(&self) -> Result<Vec<User>> {
        self.user_dao.get_multiple(doc! {}).await
    }

    pub async fn get_user_by_id(&self, user_id: ObjectId) -> Result<User> {
        self.user_dao.get_one(doc! {"_id": user_id}).await
    }

    pub async fn get_user_by_login_info(&self, username: &str, password: &str) -> Result<User> {
        self.user_dao
            .get_one(doc! {"username": username, "password": password})
            .await
    }
}

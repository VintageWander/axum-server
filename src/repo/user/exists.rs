use mongodb::bson::doc;

use crate::Result;

use super::UserRepo;

impl UserRepo {
    pub async fn exists_user_by_username(&self, username: &str) -> Result<bool> {
        self.user_dao.exists_one(doc! {"username": username}).await
    }

    pub async fn exists_user_by_email(&self, email: &str) -> Result<bool> {
        self.user_dao.exists_one(doc! {"email": email}).await
    }
}

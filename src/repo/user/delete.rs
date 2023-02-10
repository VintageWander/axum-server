use mongodb::bson::doc;

use crate::{model::user::User, Result};

use super::UserRepo;

impl UserRepo {
    pub async fn delete_user(&self, user: User) -> Result<()> {
        self.user_dao.delete_one(doc! {"_id": user.id}).await
    }
}

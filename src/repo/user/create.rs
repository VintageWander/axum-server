use crate::{model::user::User, Result};

use super::UserRepo;

impl UserRepo {
    pub async fn create_user(&self, user: User) -> Result<User> {
        self.user_dao.create_one(user).await
    }
}

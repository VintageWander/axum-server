use crate::{model::user::User, Result};

use super::UserRepo;

impl UserRepo {
    pub async fn update_user(&self, user: User) -> Result<User> {
        self.user_dao.update_one(user).await
    }
}

use crate::{model::user::User, Result};

use super::UserRepo;

impl UserRepo {
    pub async fn delete_user(&self, user: User) -> Result<User> {
        self.user_dao.delete_one(User::id(user.id)).await
    }
}

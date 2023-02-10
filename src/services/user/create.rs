use crate::{model::user::User, Result};

use super::UserService;

impl UserService {
    pub async fn create_user(&self, user: User) -> Result<User> {
        self.user_repo.create_user(user).await
    }
}

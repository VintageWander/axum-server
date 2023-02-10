use crate::{model::user::User, Result};

use super::UserService;

impl UserService {
    pub async fn update_user(&self, user: User) -> Result<User> {
        self.user_repo.update_user(user).await
    }
}

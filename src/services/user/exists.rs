use crate::{validation::user::check_username, Result};

use super::UserService;

impl UserService {
    pub async fn exists_user_by_username(&self, username: &str) -> Result<bool> {
        check_username(username)?;
        self.user_repo.exists_user_by_username(username).await
    }

    pub async fn exists_user_by_email(&self, email: &str) -> Result<bool> {
        self.user_repo.exists_user_by_email(email).await
    }
}

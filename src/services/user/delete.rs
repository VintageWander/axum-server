use crate::{model::user::User, Result};

use super::UserService;

impl UserService {
    pub async fn delete_user(&self, user: User) -> Result<User> {
        self.folder_service
            .delete_root_folder(&user)
            .await?;
        self.user_repo.delete_user(user).await
    }
}

use crate::{model::user::User, services::Service, Result};

impl Service {
    pub async fn delete_user(&self, user: User) -> Result<User> {
        self.folder_repo.delete_root_folder(&user).await?;
        self.user_repo.delete_user(user).await
    }
}

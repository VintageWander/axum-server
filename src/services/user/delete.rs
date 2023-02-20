use tokio::try_join;

use crate::{model::user::User, Result};

use super::UserService;

impl UserService {
    pub async fn delete_user(&self, user: User) -> Result<User> {
        try_join!(self.folder_service.delete_root_folder(&user))?;
        self.user_repo.delete_user(user).await
    }
}

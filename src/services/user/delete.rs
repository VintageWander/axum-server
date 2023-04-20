use crate::{model::user::User, services::Service, Result};

impl Service {
    pub async fn delete_user(&self, user: User) -> Result<User> {
        self.delete_root_folder(&user).await?;
        self.user_dao.delete_one(user).await
    }
}

use crate::{model::user::User, service::Service, Result};

impl Service {
    pub async fn delete_user(&self, user: User) -> Result<User> {
        self.delete_root_folder(&user).await?;
        self.unlink_files_from_collaborator(user.id)
            .await?;
        self.user.delete_one(user).await
    }
}

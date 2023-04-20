use tokio::try_join;

use crate::{error::Error, model::user::User, services::Service, Result};

impl Service {
    pub async fn create_user(&self, user: User) -> Result<User> {
        let (is_email_exists, is_username_exists) = try_join!(
            self.exists_user_by_email(&user.email),
            self.exists_user_by_username(&user.username),
        )?;

        if is_email_exists || is_username_exists {
            return Err(Error::ConflictUser);
        }
        let new_user = self.user_dao.insert_one(user).await?;

        // Create a root folder
        self.create_root_folder(&new_user).await?;

        Ok(new_user)
    }
}

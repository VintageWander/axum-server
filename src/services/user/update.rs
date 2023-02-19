use tokio::try_join;

use crate::{error::Error, model::user::User, Result};

use super::UserService;

impl UserService {
    pub async fn update_user(&self, user: User) -> Result<User> {
        // Get the old user
        let old_user = self.user_repo.get_user_by_id(&user.id).await?;

        // See if the new username and the new email conflicts
        let (is_email_exists, is_username_exists) = try_join!(
            self.user_repo.exists_user_by_email(&user.email),
            self.user_repo
                .exists_user_by_username(&user.username)
        )?;

        if (old_user.username != user.username && is_username_exists)
            || (old_user.email != user.email && is_email_exists)
        {
            return Err(Error::ConflictUser);
        }
        self.user_repo.update_user(user).await
    }
}

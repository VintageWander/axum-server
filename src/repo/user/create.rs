use tokio::try_join;

use crate::{error::Error, model::user::User, Result};

use super::UserRepo;

impl UserRepo {
    pub async fn create_user(&self, user: User) -> Result<User> {
        let (is_email_exists, is_username_exists) = try_join!(
            self.exists_user_by_email(&user.email),
            self.exists_user_by_username(&user.username)
        )?;

        if is_email_exists || is_username_exists {
            return Err(Error::ConflictUser);
        }
        self.user_dao.create_one(user).await
    }
}

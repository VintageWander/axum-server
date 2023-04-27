use tokio::try_join;

use crate::{error::Error, model::user::User, service::Service, Result};

impl Service {
    pub async fn update_user(&self, user: User) -> Result<User> {
        // Get the old user
        let old_user = self.get_user_by_id(user.id).await?;

        // See if the new username and the new email conflicts
        let (is_email_exists, is_username_exists) = try_join!(
            self.exists_user_by_email(&user.email),
            self.exists_user_by_username(&user.username)
        )?;

        if (old_user.username != user.username && is_username_exists)
            || (old_user.email != user.email && is_email_exists)
        {
            return Err(Error::ConflictUser);
        }

        // If the username is changed, update the root folder
        if old_user.username != user.username {
            // Get the root folder
            let mut root_folder = self
                .get_folder_by_fullpath(&format!("{}/", &old_user.username))
                .await?;

            // Change the fullpath and the folder name
            root_folder.folder_name = format!("{}/", &user.username);
            root_folder.position = format!("{}/", &user.username);
            root_folder.fullpath = format!("{}/", &user.username);

            // Update the root folder
            self.update_folder(root_folder).await?;
        }

        self.user.update_one(user).await
    }
}

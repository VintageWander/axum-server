use crate::{
    model::{folder::Folder, user::User},
    Result,
};

use super::FolderRepo;

impl FolderRepo {
    pub async fn create_folder(&self, folder: Folder) -> Result<Folder> {
        self.folder_dao.create_one(folder).await
    }

    // This function is called when a new user is created
    // So that everything has a parent folder
    pub async fn create_root_folder(&self, owner: &User) -> Result<()> {
        self.folder_dao
            .create_one(Folder::new_root(owner)?)
            .await?;
        Ok(())
    }
}

use crate::{
    model::{folder::Folder, user::User},
    Result,
};

use super::FolderService;

impl FolderService {
    pub async fn delete_folder(&self, folder: Folder) -> Result<()> {
        let deleted_folder = self.folder_repo.delete_folder(folder).await?;

        self.folder_repo
            .delete_folders_by_prefix_fullpath(&deleted_folder.fullpath)
            .await?;

        Ok(())
    }

    pub async fn delete_root_folder(&self, owner: &User) -> Result<()> {
        self.folder_repo.delete_root_folder(owner).await
    }
}

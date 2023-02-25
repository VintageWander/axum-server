use futures_util::future::try_join_all;

use crate::{
    model::{folder::Folder, user::User},
    Result,
};

use super::FolderService;

impl FolderService {
    pub async fn delete_folder(&self, folder: Folder) -> Result<()> {
        // Delete the main folder
        let deleted_folder = self.folder_repo.delete_folder(folder).await?;

        // Get all of the child folders
        // I mean ALL of them, since we're doing a regex search
        let child_folders = self
            .folder_repo
            .get_folders_by_prefix_fullpath(&deleted_folder.fullpath)
            .await?;

        // Create a future vec
        let mut tasks = vec![];

        // Interate through them
        for folder in child_folders {
            tasks.push(self.file_service.delete_files_by_folder(folder))
        }

        try_join_all(tasks).await?;

        self.folder_repo
            .delete_folders_by_prefix_fullpath(&deleted_folder.fullpath)
            .await?;

        Ok(())
    }

    pub async fn delete_root_folder(&self, owner: &User) -> Result<()> {
        // Get the root_folder
        let root_folder = self.folder_repo.get_root_folder(owner).await?;

        // This calls the delete function above, to actually clean up things,
        // both the child folders, child files, file versions, and the folder table
        self.delete_folder(root_folder).await?;

        Ok(())
    }
}

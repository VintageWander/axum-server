use mongodb::bson::doc;
use tokio::try_join;

use crate::{error::Error, model::folder::Folder, Result};

use super::FolderRepo;

impl FolderRepo {
    pub async fn update_folder(&self, folder: Folder) -> Result<Folder> {
        let folder_id = folder.id;

        let old_folder = self.get_folder_by_id(&folder_id).await?;

        if old_folder.fullpath != folder.fullpath {
            let (is_folder_conflict, exists_parent_folder) = try_join!(
                self.exists_folder_by_fullpath(&folder.fullpath), // Check for conflicts
                self.exists_folder_by_fullpath(&folder.position) // Check for parent folder existence
            )?;

            if is_folder_conflict {
                return Err(Error::ConflictFolder);
            }

            if !exists_parent_folder {
                return Err(Error::ParentFolderNotFound);
            }

            if old_folder.position == folder.position {
                return Err(Error::MoveToSelf);
            }

            if old_folder.fullpath.matches('/').count() == folder.fullpath.matches('/').count() {
                // This indicates a folder rename
                // What this check means is that if the amount of '/' are equal
                // Then it means the user is renaming a folder
                // from: User/folder
                // to :  User/folder2

                try_join!(self.change_inner_folders_position(
                    &old_folder.fullpath,
                    &old_folder.fullpath,
                    &folder.fullpath
                ))?;
            } else {
                try_join!(self.change_inner_folders_position(
                    &old_folder.fullpath,
                    &old_folder.position,
                    &folder.position
                ))?;
            }
        }
        self.folder_dao
            .update_one(doc! {"_id": folder_id}, folder)
            .await
    }
}

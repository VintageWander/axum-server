use tokio::try_join;

use crate::{error::Error, model::folder::Folder, service::Service, Result};

impl Service {
    pub async fn update_folder(&self, folder: Folder) -> Result<Folder> {
        let old_folder = self.get_folder_by_id(folder.id).await?;

        if folder.folder_name == folder.fullpath && folder.fullpath == folder.position {
            // This complicated check is used to bypass normal security when updating
            // the user's root folder
            try_join!(
                self.change_inner_folders_position(
                    &old_folder.fullpath,
                    &old_folder.fullpath,
                    &folder.fullpath
                ),
                self.change_inner_files_position(
                    &old_folder.fullpath,
                    &old_folder.fullpath,
                    &folder.fullpath
                )
            )?;
        } else if old_folder.fullpath != folder.fullpath {
            let (is_folder_conflict, exists_parent_folder) = try_join!(
                self.exists_folder_by_fullpath(&folder.fullpath), // Check for conflicts
                self.exists_folder_by_fullpath(&folder.position), // Check for parent folder existence
            )?;

            if is_folder_conflict {
                return Err(Error::ConflictFolder);
            }

            if !exists_parent_folder {
                return Err(Error::ParentFolderNotFound);
            }

            if old_folder.fullpath == folder.position {
                return Err(Error::MoveToSelf);
            }

            try_join!(
                self.change_inner_folders_position(
                    &old_folder.fullpath,
                    &old_folder.fullpath,
                    &folder.fullpath
                ),
                self.change_inner_files_position(
                    &old_folder.fullpath,
                    &old_folder.fullpath,
                    &folder.fullpath
                )
            )?;
        }
        self.folder.update_one(folder).await
    }
}

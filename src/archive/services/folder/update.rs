use tokio::try_join;

use super::FolderService;
use crate::{error::Error, model::folder::Folder, Result};

impl FolderService {
    pub async fn update_folder(&self, folder: Folder) -> Result<Folder> {
        let old_folder = self
            .folder_repo
            .get_folder_by_id(folder.id)
            .await?;

        if old_folder.fullpath != folder.fullpath {
            let (is_folder_conflict, exists_parent_folder) = try_join!(
                self.folder_repo
                    .exists_folder_by_fullpath(&folder.fullpath), // Check for conflicts
                self.folder_repo
                    .exists_folder_by_fullpath(&folder.position) // Check for parent folder existence
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
                self.folder_repo.change_inner_folders_position(
                    &old_folder.fullpath,
                    &old_folder.fullpath,
                    &folder.fullpath
                ),
                self.service.change_inner_files_position(
                    &old_folder.fullpath,
                    &old_folder.fullpath,
                    &folder.fullpath
                )
            )?;
        }
        self.folder_repo.update_folder(folder).await
    }
}

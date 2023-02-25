use mongodb::bson::oid::ObjectId;

use super::FolderService;
use crate::validation::file::check_dir;
use crate::Result;

impl FolderService {
    pub async fn exists_folder_by_id(&self, folder_id: ObjectId) -> Result<bool> {
        self.folder_repo
            .exists_folder_by_id(folder_id)
            .await
    }

    // Useful to check if a folder exists when creating a new folder
    // Prevent conflicts
    // (This function basically check conflicts before creating a new folder)
    pub async fn exists_folder_by_fullpath(&self, fullpath: &str) -> Result<bool> {
        check_dir(fullpath)?;
        self.folder_repo
            .exists_folder_by_fullpath(fullpath)
            .await
    }

    // Checks if a folder exists at a given position,
    // before adding a new folder as a child folder
    // (It basically means check if parent exists)
    pub async fn exists_folder_by_position(&self, position: &str) -> Result<bool> {
        check_dir(position)?;
        self.folder_repo
            .exists_folder_by_position(position)
            .await
    }
}

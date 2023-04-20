use mongodb::bson::oid::ObjectId;

use crate::{model::folder::Folder, services::Service, validation::file::check_dir, Result};

impl Service {
    pub async fn exists_folder_by_id(&self, folder_id: ObjectId) -> Result<bool> {
        self.folder_dao
            .exists_one(Folder::id(folder_id))
            .await
    }

    // Useful to check if a folder exists when creating a new folder
    // Prevent conflicts
    // (This function basically check conflicts before creating a new folder)
    pub async fn exists_folder_by_fullpath(&self, fullpath: &str) -> Result<bool> {
        check_dir(fullpath)?;

        self.folder_dao
            .exists_one(Folder::fullpath(fullpath))
            .await
    }

    // Checks if a folder exists at a given position,
    // before adding a new folder as a child folder
    // (It basically means check if parent exists)
    pub async fn exists_folder_by_position(&self, position: &str) -> Result<bool> {
        check_dir(position)?;

        self.folder_dao
            .exists_one(Folder::position(position))
            .await
    }
}

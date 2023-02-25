use mongodb::bson::{oid::ObjectId, Document};

use crate::{
    model::{folder::Folder, user::User},
    services::folder::FolderService,
    validation::file::check_dir,
    Result,
};

impl FolderService {
    pub async fn get_folders_by(&self, doc: Document) -> Result<Vec<Folder>> {
        self.folder_repo.get_folders_by(doc).await
    }

    pub async fn get_folders_by_owner(&self, owner: &User) -> Result<Vec<Folder>> {
        self.folder_repo.get_folders_by_owner(owner).await
    }

    pub async fn get_folder_by_id(&self, folder_id: ObjectId) -> Result<Folder> {
        self.folder_repo.get_folder_by_id(folder_id).await
    }

    pub async fn get_folder_by_id_owner(
        &self,
        folder_id: ObjectId,
        owner: &User,
    ) -> Result<Folder> {
        self.folder_repo
            .get_folder_by_id_owner(folder_id, owner)
            .await
    }

    // This function is useful for getting all folders at a given location
    // to create a folder tree
    pub async fn get_folders_by_position(&self, position: &str) -> Result<Vec<Folder>> {
        check_dir(position)?;
        self.folder_repo
            .get_folders_by_position(position)
            .await
    }

    pub async fn get_folder_by_fullpath(&self, fullpath: &str) -> Result<Folder> {
        check_dir(fullpath)?;
        self.folder_repo
            .get_folder_by_fullpath(fullpath)
            .await
    }

    pub async fn get_folders_by_prefix_fullpath(&self, fullpath: &str) -> Result<Vec<Folder>> {
        check_dir(fullpath)?;
        self.folder_repo
            .get_folders_by_prefix_fullpath(fullpath)
            .await
    }
}

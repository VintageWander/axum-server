use std::vec::IntoIter;

use mongodb::bson::{oid::ObjectId, Document};

use crate::{
    model::{folder::Folder, user::User},
    validation::file::check_dir,
    Result,
};

use super::FolderService;

impl FolderService {
    pub async fn get_folders_by(&self, doc: Document) -> Result<IntoIter<Folder>> {
        self.folder_repo.get_folders_by(doc).await
    }

    pub async fn get_folder_by_id(&self, folder_id: &ObjectId) -> Result<Folder> {
        self.folder_repo.get_folder_by_id(folder_id).await
    }

    pub async fn get_folders_by_owner(&self, owner: &User) -> Result<IntoIter<Folder>> {
        self.folder_repo.get_folders_by_owner(owner).await
    }

    pub async fn get_public_folders_by_owner(&self, owner: &User) -> Result<IntoIter<Folder>> {
        self.folder_repo.get_public_folders_by_owner(owner).await
    }

    pub async fn get_shared_folders_by_owner(&self, owner: &User) -> Result<IntoIter<Folder>> {
        self.folder_repo.get_shared_folders_by_owner(owner).await
    }

    // This function is useful for getting all folders at a given location
    // to create a folder tree
    pub async fn get_folders_by_position(&self, position: &str) -> Result<IntoIter<Folder>> {
        check_dir(position)?;
        self.folder_repo.get_folders_by_position(position).await
    }

    pub async fn get_public_folders_by_position(&self, position: &str) -> Result<IntoIter<Folder>> {
        check_dir(position)?;
        self.folder_repo
            .get_public_folders_by_position(position)
            .await
    }

    // This is useful for peeking the inner contents of a specific folder
    pub async fn get_folders_by_fullpath(&self, fullpath: &str) -> Result<IntoIter<Folder>> {
        check_dir(fullpath)?;
        self.folder_repo.get_folders_by_fullpath(fullpath).await
    }
}

use std::vec::IntoIter;

use mongodb::bson::{doc, oid::ObjectId, Document};

use crate::{
    model::{folder::Folder, user::User},
    validation::file::check_dir,
    Result,
};

use super::FolderRepo;

impl FolderRepo {
    pub async fn get_folders_by(&self, doc: Document) -> Result<IntoIter<Folder>> {
        self.folder_dao.get_multiple(doc).await
    }

    pub async fn get_folder_by_id(&self, folder_id: &ObjectId) -> Result<Folder> {
        self.folder_dao.get_one(doc! {"_id": folder_id}).await
    }

    pub async fn get_public_folders(&self) -> Result<IntoIter<Folder>> {
        self.folder_dao
            .get_multiple(doc! {"visibility": "public"})
            .await
    }

    pub async fn get_folders_by_owner(&self, owner: &User) -> Result<IntoIter<Folder>> {
        self.folder_dao.get_multiple(doc! {"owner": owner.id}).await
    }

    pub async fn get_public_folders_by_owner(&self, owner: &User) -> Result<IntoIter<Folder>> {
        self.folder_dao
            .get_multiple(doc! {"visibility": "public" ,"owner": owner.id})
            .await
    }

    pub async fn get_shared_folders_by_owner(&self, owner: &User) -> Result<IntoIter<Folder>> {
        self.folder_dao
            .get_multiple(doc! {"visibility": "shared", "owner": owner.id})
            .await
    }

    // This function is useful for getting all folders at a given location
    // to create a folder tree
    pub async fn get_folders_by_position(&self, position: &str) -> Result<IntoIter<Folder>> {
        check_dir(position)?;
        self.folder_dao
            .get_multiple(doc! {"position": position})
            .await
    }

    pub async fn get_public_folders_by_position(&self, position: &str) -> Result<IntoIter<Folder>> {
        check_dir(position)?;
        self.folder_dao
            .get_multiple(doc! {"visibility": "public", "position": position})
            .await
    }

    // This is useful for peeking the inner contents of a specific folder
    pub async fn get_folders_by_fullpath(&self, fullpath: &str) -> Result<IntoIter<Folder>> {
        check_dir(fullpath)?;
        self.folder_dao
            .get_multiple(doc! {"visibility": "public", "fullpath": fullpath})
            .await
    }
}

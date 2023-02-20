use std::vec::IntoIter;

use mongodb::bson::{doc, oid::ObjectId, Document};

use crate::{
    model::{folder::Folder, user::User},
    Result,
};

use super::FolderRepo;

impl FolderRepo {
    // This repository works as like an extension on top of the dao layer
    // Validation will be performed at that service layer
    pub async fn get_folders_by(&self, doc: Document) -> Result<IntoIter<Folder>> {
        self.folder_dao.get_multiple(doc).await
    }

    pub async fn get_folder_by_id(&self, folder_id: &ObjectId) -> Result<Folder> {
        self.folder_dao
            .get_one(doc! {"_id": folder_id})
            .await
    }

    pub async fn get_folder_by_id_owner(
        &self,
        folder_id: &ObjectId,
        owner: &User,
    ) -> Result<Folder> {
        self.folder_dao
            .get_one(doc! {"_id": folder_id, "owner": owner.id})
            .await
    }

    pub async fn get_folders(&self) -> Result<IntoIter<Folder>> {
        self.folder_dao.get_multiple(doc! {}).await
    }

    pub async fn get_public_folders(&self) -> Result<IntoIter<Folder>> {
        self.folder_dao
            .get_multiple(doc! {"visibility": "public"})
            .await
    }

    pub async fn get_folders_by_owner(&self, owner: &User) -> Result<IntoIter<Folder>> {
        self.folder_dao
            .get_multiple(doc! {"owner": owner.id})
            .await
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
        self.folder_dao
            .get_multiple(doc! {"position": position})
            .await
    }

    pub async fn get_public_folders_by_position(&self, position: &str) -> Result<IntoIter<Folder>> {
        self.folder_dao
            .get_multiple(doc! {"visibility": "public", "position": position})
            .await
    }

    // Since the fullpath is always unique
    // This function returns one folder
    pub async fn get_folder_by_fullpath(&self, fullpath: &str) -> Result<Folder> {
        self.folder_dao
            .get_one(doc! {"fullpath": fullpath})
            .await
    }

    // Since the fullpath is always unique
    // This function returns one folder
    pub async fn get_public_folder_by_fullpath(&self, fullpath: &str) -> Result<Folder> {
        self.folder_dao
            .get_one(doc! {"visibility": "public", "fullpath": fullpath})
            .await
    }
}

use crate::{
    model::{folder::Folder, user::User},
    repo::folder::FolderRepo,
    Result,
};

use mongodb::bson::doc;

impl FolderRepo {
    pub async fn get_public_folders(&self) -> Result<Vec<Folder>> {
        self.folder_dao
            .get_many(doc! {"visibility": "public"})
            .await
    }

    pub async fn get_public_folders_by_owner(&self, owner: &User) -> Result<Vec<Folder>> {
        self.folder_dao
            .get_many(doc! {"visibility": "public" ,"owner": owner.id})
            .await
    }

    pub async fn get_public_folders_by_position(&self, position: &str) -> Result<Vec<Folder>> {
        self.folder_dao
            .get_many(doc! {"visibility": "public", "position": position})
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

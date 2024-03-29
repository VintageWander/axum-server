use mongodb::bson::oid::ObjectId;

use crate::{
    model::{
        folder::{Folder, FolderVisibility},
        user::User,
    },
    service::Service,
    validation::file::check_dir,
    Result,
};

impl Service {
    pub async fn get_public_folders(&self) -> Result<Vec<Folder>> {
        self.folder
            .get_many(Folder::visibility(FolderVisibility::Public))
            .await
    }
    pub async fn get_public_folders_by_owner(&self, owner: &User) -> Result<Vec<Folder>> {
        self.folder
            .get_many(Folder::visibility(FolderVisibility::Public).owner(owner.id))
            .await
    }

    pub async fn get_public_folders_by_position(&self, position: &str) -> Result<Vec<Folder>> {
        check_dir(position)?;

        self.folder
            .get_many(Folder::visibility(FolderVisibility::Public).position(position))
            .await
    }

    pub async fn get_public_folder_by_id(&self, folder_id: ObjectId) -> Result<Folder> {
        self.folder.get_one(Folder::id(folder_id)).await
    }

    pub async fn get_public_folder_by_fullpath(&self, fullpath: &str) -> Result<Folder> {
        check_dir(fullpath)?;

        self.folder
            .get_one(Folder::visibility(FolderVisibility::Public).fullpath(fullpath))
            .await
    }
}

use mongodb::bson::{doc, oid::ObjectId, Document, Regex};

use crate::{
    model::{folder::Folder, user::User},
    repo::folder::FolderRepo,
    validation::file::check_dir,
    Result,
};

impl FolderRepo {
    // This repository works as like an extension on top of the dao layer
    // Validation will be performed at that service layer
    pub async fn get_folders_by(&self, doc: Document) -> Result<Vec<Folder>> {
        self.folder_dao.get_multiple(doc).await
    }

    pub async fn get_folder_by_id(&self, folder_id: ObjectId) -> Result<Folder> {
        self.folder_dao
            .get_one(doc! {"_id": folder_id})
            .await
    }

    pub async fn get_folder_by_id_owner(
        &self,
        folder_id: ObjectId,
        owner: &User,
    ) -> Result<Folder> {
        self.folder_dao
            .get_one(doc! {"_id": folder_id, "owner": owner.id})
            .await
    }

    pub async fn get_folders_by_owner(&self, owner: &User) -> Result<Vec<Folder>> {
        self.folder_dao
            .get_multiple(doc! {"owner": owner.id})
            .await
    }

    pub async fn get_root_folder(&self, owner: &User) -> Result<Folder> {
        self.folder_dao
            .get_one(doc! {
                "folderName": format!("{}/", owner.username),
                "fullpath": format!("{}/", owner.username)
            })
            .await
    }

    // This function is useful for getting all folders at a given location
    // to create a folder tree
    pub async fn get_folders_by_position(&self, position: &str) -> Result<Vec<Folder>> {
        self.folder_dao
            .get_multiple(doc! {"position": position})
            .await
    }

    // Since the fullpath is always unique
    // This function returns one folder
    pub async fn get_folder_by_fullpath(&self, fullpath: &str) -> Result<Folder> {
        self.folder_dao
            .get_one(doc! {"fullpath": fullpath})
            .await
    }

    pub async fn get_folders_by_prefix_fullpath(&self, fullpath: &str) -> Result<Vec<Folder>> {
        check_dir(fullpath)?;
        let fullpath_regex = Regex {
            pattern: format!("^{fullpath}"),
            options: String::new(),
        };
        self.folder_dao
            .get_multiple(doc! {"fullpath": {
                    "$regex": fullpath_regex
                }
            })
            .await
    }
}

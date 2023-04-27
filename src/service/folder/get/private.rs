use mongodb::bson::{doc, oid::ObjectId, Document, Regex};

use crate::{
    model::{folder::Folder, user::User},
    service::Service,
    validation::file::check_dir,
    Result,
};

impl Service {
    pub async fn get_folders_by(&self, doc: Document) -> Result<Vec<Folder>> {
        self.folder.get_many(doc).await
    }

    pub async fn get_folders_by_owner(&self, owner: &User) -> Result<Vec<Folder>> {
        self.folder
            .get_many(Folder::owner(owner.id))
            .await
    }

    pub async fn get_folder_by_id(&self, folder_id: ObjectId) -> Result<Folder> {
        self.folder.get_one(Folder::id(folder_id)).await
    }

    pub async fn get_folder_by_id_owner(
        &self,
        folder_id: ObjectId,
        owner: &User,
    ) -> Result<Folder> {
        self.folder
            .get_one(Folder::id(folder_id).owner(owner.id))
            .await
    }

    // This function is useful for getting all folders at a given location
    // to create a folder tree
    pub async fn get_folders_by_position(&self, position: &str) -> Result<Vec<Folder>> {
        check_dir(position)?;

        self.folder
            .get_many(Folder::position(position))
            .await
    }

    pub async fn get_folder_by_fullpath(&self, fullpath: &str) -> Result<Folder> {
        check_dir(fullpath)?;

        self.folder
            .get_one(Folder::fullpath(fullpath))
            .await
    }

    pub async fn get_folders_by_prefix_fullpath(&self, fullpath: &str) -> Result<Vec<Folder>> {
        check_dir(fullpath)?;

        let fullpath_regex = Regex {
            pattern: format!("^{fullpath}"),
            options: String::new(),
        };

        self.folder
            .get_many(doc! {"fullpath": {
                    "$regex": fullpath_regex
                }
            })
            .await
    }

    pub async fn get_root_folder(&self, owner: &User) -> Result<Folder> {
        self.folder
            .get_one(doc! {
                "folderName": format!("{}/", owner.username),
                "fullpath": format!("{}/", owner.username)
            })
            .await
    }
}

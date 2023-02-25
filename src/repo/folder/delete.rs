use mongodb::bson::{doc, Regex};

use crate::{
    model::{folder::Folder, user::User},
    validation::file::check_dir,
    Result,
};

use super::FolderRepo;

impl FolderRepo {
    // The only usage of this function is in the delete method below
    // It only lives in the repository layer,
    // I do not expose it to the service layer because there are no use for it
    pub async fn delete_folders_by_prefix_fullpath(&self, fullpath: &str) -> Result<()> {
        check_dir(fullpath)?;
        let fullpath_regex = Regex {
            pattern: format!("^{fullpath}"),
            options: String::new(),
        };
        self.folder_dao
            .delete_multiple(doc! {"fullpath": {
                    "$regex": fullpath_regex
                }
            })
            .await
    }

    pub async fn delete_folder(&self, folder: Folder) -> Result<Folder> {
        self.folder_dao
            .delete_one(doc! {"_id": folder.id})
            .await
    }

    // This is more like "delete_folder_by_owner"
    pub async fn delete_root_folder(&self, owner: &User) -> Result<Folder> {
        self.folder_dao
            .delete_one(doc! {
                "folderName": format!("{}/", owner.username),
                "fullpath": format!("{}/", owner.username)
            })
            .await
    }
}

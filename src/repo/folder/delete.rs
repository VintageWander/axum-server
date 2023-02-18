use mongodb::bson::{doc, Regex};

use crate::{error::Error, model::folder::Folder, validation::file::check_dir, Result};

use super::FolderRepo;

impl FolderRepo {
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

    pub async fn delete_folder(&self, folder: Folder) -> Result<()> {
        // The root folder is the only folder that has the position and the fullpath equal
        // Prevents user from deleting the root folder
        if folder.position == folder.fullpath {
            return Err(Error::Unauthorized);
        }

        let folder_id = folder.id;

        let deleted_folder = self.folder_dao.delete_one(doc! {"_id": folder_id}).await?;

        self.delete_folders_by_prefix_fullpath(&deleted_folder.fullpath)
            .await?;

        todo!()
    }
}

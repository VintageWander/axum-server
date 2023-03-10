use mongodb::bson::doc;

use crate::{model::folder::Folder, Result};

use super::FolderRepo;

impl FolderRepo {
    pub async fn update_folder(&self, folder: Folder) -> Result<Folder> {
        self.folder_dao
            .update_one(doc! {"_id": folder.id}, folder)
            .await
    }
}

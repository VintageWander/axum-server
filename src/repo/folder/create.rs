use crate::{model::folder::Folder, Result};

use super::FolderRepo;

impl FolderRepo {
    pub async fn create_folder(&self, folder: Folder) -> Result<Folder> {
        self.folder_dao.create_one(folder).await
    }
}

use crate::{model::folder::Folder, Result};

use super::FolderService;

impl FolderService {
    pub async fn delete_folder(&self, folder: Folder) -> Result<()> {
        self.folder_repo.delete_folder(folder).await
    }
}

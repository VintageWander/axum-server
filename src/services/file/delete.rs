use futures_util::future::try_join_all;
use tokio::try_join;

use crate::{
    model::{file::File, folder::Folder},
    services::Service,
    Result,
};

impl Service {
    pub async fn delete_file(&self, file: File) -> Result<File> {
        // This function will delete all version files
        self.delete_versions_by_file(&file).await?;

        // Create the file path
        let file_path = format!("{}.{}", file.id, file.extension.to_string());

        let (_, _, deleted_file) = try_join!(
            self.storage.delete_object(file_path),
            self.storage.delete_folder(file.id.to_string()),
            self.file_dao.delete_one(file)
        )?;
        Ok(deleted_file)
    }

    pub async fn delete_files_by_folder(&self, folder: Folder) -> Result<()> {
        let files = self
            .get_files_by_position(&folder.fullpath)
            .await?;

        let mut tasks = vec![];

        for file in files {
            tasks.push(self.delete_file(file))
        }

        try_join_all(tasks).await?;

        Ok(())
    }
}

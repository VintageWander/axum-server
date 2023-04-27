use futures_util::future::try_join_all;
use tokio::try_join;

use crate::{
    model::{file::File, folder::Folder, user::User},
    service::Service,
    Result,
};

impl Service {
    pub async fn delete_file(&self, file: File) -> Result<File> {
        // This function will delete all version files
        self.delete_versions_by_file(&file).await?;

        // Create the file path
        let file_path = format!("{}.{}", file.id, file.extension.to_string());

        let (_, _, _, deleted_file) = try_join!(
            self.unlink_collaborators_from_file(file.id),
            self.storage.delete_object(file_path),
            self.storage.delete_folder(file.id.to_string()),
            self.file.delete_one(file)
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

    pub async fn remove_collaborator_from_file(
        &self,
        collaborator: &User,
        file: &File,
    ) -> Result<()> {
        self.unlink_file_collaborator(file.id, collaborator.id)
            .await?;
        Ok(())
    }
}

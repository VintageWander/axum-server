use futures_util::future::try_join_all;

use crate::{
    model::{file::File, file_version::FileVersion},
    Result,
};

use super::FileVersionService;

impl FileVersionService {
    pub async fn delete_version_by_id_file(
        &self,
        version_number: i64,
        file: &File,
    ) -> Result<FileVersion> {
        // Replicate file version path
        let file_version_path = &format!(
            "{}/{}.{}",
            file.id,
            version_number,
            file.extension.to_string()
        );

        self.storage
            .delete_object(file_version_path)
            .await?;

        self.file_version_repo
            .delete_version(version_number)
            .await
    }

    pub async fn delete_versions_by_file(&self, file: &File) -> Result<()> {
        // Get all versions of a file
        let file_versions = self
            .file_version_repo
            .get_file_versions(file)
            .await?;

        // Create a tasks vector
        let mut tasks = vec![];

        // Iterator through the list of version and delete them one by one
        for FileVersion { version_number, .. } in file_versions {
            tasks.push(self.delete_version_by_id_file(version_number, file))
        }

        // Execute the tasks vector
        try_join_all(tasks).await?;
        Ok(())
    }
}

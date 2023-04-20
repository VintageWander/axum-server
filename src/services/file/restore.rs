use crate::{model::file::File, services::Service, Result};

impl Service {
    pub async fn restore_file(&self, file: &File, version_number: i64) -> Result<()> {
        // First we need to back up the current file

        self.create_version(file).await?;

        // Get the FileVersion from the version_number provided
        let restore_version = self.get_version_by_number(version_number).await?;

        // Construct the file version path
        let file_version_path = &format!(
            "{}/{}.{}",
            file.id,
            restore_version.version_number,
            file.extension.to_string()
        );

        // Construct the current file path
        let current_file_path = &format!("{}.{}", file.id, file.extension.to_string());

        self.storage
            .move_object(file_version_path, current_file_path)
            .await?;
        self.delete_version_by_id_file(version_number, file)
            .await?;

        Ok(())
    }
}

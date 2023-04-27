use chrono::Utc;

use crate::{
    model::{file::File, file_version::FileVersion},
    service::Service,
    Result,
};

impl Service {
    pub async fn create_version(&self, file: &File) -> Result<FileVersion> {
        // Create a version number
        let version_number = Utc::now().timestamp_millis();

        // Instantiate version path
        // <file_id>/<version_number>.<file extension>

        let file_version_path = &format!(
            "{}/{}.{}",
            file.id,
            version_number,
            file.extension.to_string()
        );

        // Get the current file path
        let current_file_path = &format!("{}.{}", file.id, file.extension.to_string());

        // Move the file from the current path to the internal file version path
        self.storage
            .move_object(current_file_path, file_version_path)
            .await?;

        self.file_version
            .insert_one(FileVersion::new(file, version_number))
            .await
    }
}

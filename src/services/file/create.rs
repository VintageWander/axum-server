use tokio::try_join;

use crate::{model::file::File, Result};

use super::FileService;

impl FileService {
    pub async fn create_file(&self, file: File, bytes: Vec<u8>) -> Result<File> {
        let full_filename = format!("{}.{}", &file.id, &file.extension.to_string());
        let (new_file, _) = try_join!(
            self.file_repo.create_file(file),
            self.storage.put_object(&full_filename, bytes)
        )?;
        Ok(new_file)
    }
}

use crate::{model::file::File, Result};

use super::FileRepo;

impl FileRepo {
    pub async fn create_file(&self, file: File) -> Result<File> {
        self.file_dao.insert_one(file).await
    }
}

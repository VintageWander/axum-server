use mongodb::bson::doc;

use crate::{model::file::File, Result};

use super::FileRepo;

impl FileRepo {
    pub async fn delete_file(&self, file: File) -> Result<File> {
        self.file_dao
            .delete_one(doc! {"_id": file.id})
            .await
    }
}

use mongodb::bson::doc;

use crate::{model::file::File, Result};

use super::FileRepo;

impl FileRepo {
    pub async fn update_file(&self, file: File) -> Result<File> {
        self.file_dao
            .update_one(doc! {"_id": file.id}, file)
            .await
    }
}

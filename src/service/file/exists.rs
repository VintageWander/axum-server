use mongodb::bson::oid::ObjectId;

use crate::{model::file::File, service::Service, validation::file::check_fullpath, Result};

impl Service {
    pub async fn exists_file_by_id(&self, file_id: ObjectId) -> Result<bool> {
        self.file.exists_one(File::id(file_id)).await
    }

    pub async fn exists_file_by_fullpath(&self, fullpath: &str) -> Result<bool> {
        check_fullpath(fullpath)?;
        self.file
            .exists_one(File::fullpath(fullpath))
            .await
    }
}

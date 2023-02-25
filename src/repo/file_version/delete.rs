use mongodb::bson::doc;

use crate::{
    model::{file::File, file_version::FileVersion},
    Result,
};

use super::FileVersionRepo;

impl FileVersionRepo {
    pub async fn delete_version(&self, version_number: i64) -> Result<FileVersion> {
        self.file_version_dao
            .delete_one(doc! {"versionNumber": version_number})
            .await
    }

    pub async fn delete_versions_by_file(&self, file: &File) -> Result<()> {
        self.file_version_dao
            .delete_multiple(doc! {"fileId": file.id})
            .await
    }
}

use mongodb::bson::doc;

use crate::{
    model::{file::File, file_version::FileVersion},
    Result,
};

use super::FileVersionRepo;

impl FileVersionRepo {
    pub async fn get_file_versions(&self, file: &File) -> Result<Vec<FileVersion>> {
        self.file_version_dao
            .get_many(doc! {"fileId": file.id})
            .await
    }

    pub async fn get_version_by_number(&self, version_number: i64) -> Result<FileVersion> {
        self.file_version_dao
            .get_one(doc! {"versionNumber": version_number})
            .await
    }
}

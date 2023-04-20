use crate::{
    model::{file::File, file_version::FileVersion},
    services::Service,
    Result,
};

impl Service {
    pub async fn get_file_versions(&self, file: &File) -> Result<Vec<FileVersion>> {
        self.file_version_dao
            .get_many(FileVersion::file_id(file.id))
            .await
    }

    pub async fn get_version_by_number(&self, version_number: i64) -> Result<FileVersion> {
        self.file_version_dao
            .get_one(FileVersion::version_number(version_number))
            .await
    }
}

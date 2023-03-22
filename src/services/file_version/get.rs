use crate::{
    model::{file::File, file_version::FileVersion},
    services::Service,
    Result,
};

impl Service {
    pub async fn get_file_versions(&self, file: &File) -> Result<Vec<FileVersion>> {
        self.file_version_repo
            .get_file_versions(file)
            .await
    }

    pub async fn get_version_by_number(&self, version_number: i64) -> Result<FileVersion> {
        self.file_version_repo
            .get_version_by_number(version_number)
            .await
    }
}

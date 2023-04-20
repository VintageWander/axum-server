use crate::{model::file::File, services::Service, validation::file::check_fullpath, Result};

impl Service {
    pub async fn exists_file_by_fullpath(&self, fullpath: &str) -> Result<bool> {
        check_fullpath(fullpath)?;
        self.file_dao
            .exists_one(File::fullpath(fullpath))
            .await
    }
}

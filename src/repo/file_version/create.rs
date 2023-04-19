use crate::{model::file_version::FileVersion, Result};

use super::FileVersionRepo;

impl FileVersionRepo {
    pub async fn create_version(&self, fv: FileVersion) -> Result<FileVersion> {
        self.file_version_dao.insert_one(fv).await
    }
}

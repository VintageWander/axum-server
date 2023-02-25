use mongodb::bson::doc;

use crate::Result;

use super::FileRepo;

impl FileRepo {
    pub async fn exists_file_by_fullpath(&self, fullpath: &str) -> Result<bool> {
        self.file_dao
            .exists_one(doc! {"fullpath": fullpath})
            .await
    }
}

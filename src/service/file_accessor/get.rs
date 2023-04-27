use mongodb::bson::oid::ObjectId;

use crate::{model::file_accessor::FileAccessor, service::Service, Result};

impl Service {
    // fa means file accessor
    // foa means folder accessor
    pub async fn get_fas_by_file_id(&self, file_id: ObjectId) -> Result<Vec<FileAccessor>> {
        self.file_accessor
            .get_many(FileAccessor::file_id(file_id))
            .await
    }

    pub async fn get_fas_by_user_id(&self, user_id: ObjectId) -> Result<Vec<FileAccessor>> {
        self.file_accessor
            .get_many(FileAccessor::user_id(user_id))
            .await
    }

    pub async fn get_fa(&self, file_id: ObjectId, user_id: ObjectId) -> Result<FileAccessor> {
        self.file_accessor
            .get_one(FileAccessor::file_id(file_id).user_id(user_id))
            .await
    }
}

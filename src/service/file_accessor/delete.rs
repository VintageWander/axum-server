use mongodb::bson::oid::ObjectId;

use crate::{model::file_accessor::FileAccessor, service::Service, Result};

impl Service {
    pub async fn unlink_file_accessor(
        &self,
        file_id: ObjectId,
        accessor_id: ObjectId,
    ) -> Result<()> {
        self.file_accessor
            .delete_one(FileAccessor::file_id(file_id).user_id(accessor_id))
            .await?;
        Ok(())
    }

    pub async fn unlink_files_from_accessor(&self, accessor_id: ObjectId) -> Result<()> {
        self.file_accessor
            .delete_many(FileAccessor::user_id(accessor_id))
            .await?;
        Ok(())
    }

    pub async fn unlink_accessors_from_file(&self, file_id: ObjectId) -> Result<()> {
        self.file_accessor
            .delete_many(FileAccessor::file_id(file_id))
            .await?;
        Ok(())
    }
}

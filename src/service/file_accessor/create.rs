use mongodb::bson::oid::ObjectId;

use crate::{model::file_accessor::FileAccessor, service::Service, Result};

impl Service {
    pub async fn link_file_accessor(
        &self,
        file_id: ObjectId,
        accessor_id: ObjectId,
    ) -> Result<FileAccessor> {
        self.file_accessor
            .insert_one(
                FileAccessor::file_id(file_id)
                    .user_id(accessor_id)
                    .build()?,
            )
            .await
    }
}

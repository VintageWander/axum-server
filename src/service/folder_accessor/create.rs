use mongodb::bson::oid::ObjectId;

use crate::{model::folder_accessor::FolderAccessor, service::Service, Result};

impl Service {
    pub async fn link_folder_accessor(
        &self,
        folder_id: ObjectId,
        accessor: ObjectId,
    ) -> Result<FolderAccessor> {
        self.folder_accessor
            .insert_one(
                FolderAccessor::folder_id(folder_id)
                    .user_id(accessor)
                    .build()?,
            )
            .await
    }
}

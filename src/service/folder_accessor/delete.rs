use mongodb::bson::oid::ObjectId;

use crate::{model::folder_accessor::FolderAccessor, service::Service, Result};

impl Service {
    pub async fn unlink_folder_accessor(
        &self,
        folder_id: ObjectId,
        accessor_id: ObjectId,
    ) -> Result<()> {
        self.folder_accessor
            .delete_one(FolderAccessor::folder_id(folder_id).user_id(accessor_id))
            .await?;
        Ok(())
    }

    pub async fn unlink_folders_from_accessor(&self, accessor_id: ObjectId) -> Result<()> {
        self.folder_accessor
            .delete_many(FolderAccessor::user_id(accessor_id))
            .await?;
        Ok(())
    }

    pub async fn unlink_accessors_from_folder(&self, folder_id: ObjectId) -> Result<()> {
        self.folder_accessor
            .delete_many(FolderAccessor::folder_id(folder_id))
            .await?;
        Ok(())
    }
}

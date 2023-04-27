use mongodb::bson::oid::ObjectId;

use crate::{model::folder_accessor::FolderAccessor, service::Service, Result};

impl Service {
    // fa means file accessor
    // foa means folder accessor
    pub async fn get_foas_by_folder_id(&self, folder_id: ObjectId) -> Result<Vec<FolderAccessor>> {
        self.folder_accessor
            .get_many(FolderAccessor::folder_id(folder_id))
            .await
    }

    pub async fn get_foas_by_user_id(&self, user_id: ObjectId) -> Result<Vec<FolderAccessor>> {
        self.folder_accessor
            .get_many(FolderAccessor::user_id(user_id))
            .await
    }
}

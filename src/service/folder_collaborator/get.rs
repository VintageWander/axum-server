use mongodb::bson::oid::ObjectId;

use crate::{model::folder_collaborator::FolderCollaborator, service::Service, Result};

impl Service {
    // fa means file collaborator
    // foa means folder collaborator
    pub async fn get_foas_by_folder_id(
        &self,
        folder_id: ObjectId,
    ) -> Result<Vec<FolderCollaborator>> {
        self.folder_collaborator
            .get_many(FolderCollaborator::folder_id(folder_id))
            .await
    }

    pub async fn get_foas_by_user_id(&self, user_id: ObjectId) -> Result<Vec<FolderCollaborator>> {
        self.folder_collaborator
            .get_many(FolderCollaborator::user_id(user_id))
            .await
    }

    pub async fn get_foa(
        &self,
        folder_id: ObjectId,
        user_id: ObjectId,
    ) -> Result<FolderCollaborator> {
        self.folder_collaborator
            .get_one(FolderCollaborator::folder_id(folder_id).user_id(user_id))
            .await
    }
}

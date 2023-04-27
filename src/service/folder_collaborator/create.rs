use mongodb::bson::oid::ObjectId;

use crate::{model::folder_collaborator::FolderCollaborator, service::Service, Result};

impl Service {
    pub async fn link_folder_collaborator(
        &self,
        folder_id: ObjectId,
        collaborator: ObjectId,
    ) -> Result<FolderCollaborator> {
        self.folder_collaborator
            .insert_one(
                FolderCollaborator::folder_id(folder_id)
                    .user_id(collaborator)
                    .build()?,
            )
            .await
    }
}

use mongodb::bson::oid::ObjectId;

use crate::{model::folder_collaborator::FolderCollaborator, service::Service, Result};

impl Service {
    pub async fn unlink_folder_collaborator(
        &self,
        folder_id: ObjectId,
        collaborator_id: ObjectId,
    ) -> Result<()> {
        self.folder_collaborator
            .delete_one(FolderCollaborator::folder_id(folder_id).user_id(collaborator_id))
            .await?;
        Ok(())
    }

    pub async fn unlink_folders_from_collaborator(&self, collaborator_id: ObjectId) -> Result<()> {
        self.folder_collaborator
            .delete_many(FolderCollaborator::user_id(collaborator_id))
            .await?;
        Ok(())
    }

    pub async fn unlink_collaborators_from_folder(&self, folder_id: ObjectId) -> Result<()> {
        self.folder_collaborator
            .delete_many(FolderCollaborator::folder_id(folder_id))
            .await?;
        Ok(())
    }
}

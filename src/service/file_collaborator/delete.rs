use mongodb::bson::oid::ObjectId;

use crate::{model::file_collaborator::FileCollaborator, service::Service, Result};

impl Service {
    pub async fn unlink_file_collaborator(
        &self,
        file_id: ObjectId,
        collaborator_id: ObjectId,
    ) -> Result<()> {
        self.file_collaborator
            .delete_one(FileCollaborator::file_id(file_id).user_id(collaborator_id))
            .await?;
        Ok(())
    }

    pub async fn unlink_files_from_collaborator(&self, collaborator_id: ObjectId) -> Result<()> {
        self.file_collaborator
            .delete_many(FileCollaborator::user_id(collaborator_id))
            .await?;
        Ok(())
    }

    pub async fn unlink_collaborators_from_file(&self, file_id: ObjectId) -> Result<()> {
        self.file_collaborator
            .delete_many(FileCollaborator::file_id(file_id))
            .await?;
        Ok(())
    }
}

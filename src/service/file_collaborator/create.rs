use mongodb::bson::oid::ObjectId;

use crate::{model::file_collaborator::FileCollaborator, service::Service, Result};

impl Service {
    pub async fn link_file_collaborator(
        &self,
        file_id: ObjectId,
        collaborator_id: ObjectId,
    ) -> Result<FileCollaborator> {
        self.file_collaborator
            .insert_one(
                FileCollaborator::file_id(file_id)
                    .user_id(collaborator_id)
                    .build()?,
            )
            .await
    }
}

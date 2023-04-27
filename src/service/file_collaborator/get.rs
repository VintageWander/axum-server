use mongodb::bson::oid::ObjectId;

use crate::{model::file_collaborator::FileCollaborator, service::Service, Result};

impl Service {
    // fa means file collaborator
    // foa means folder collaborator
    pub async fn get_fas_by_file_id(&self, file_id: ObjectId) -> Result<Vec<FileCollaborator>> {
        self.file_collaborator
            .get_many(FileCollaborator::file_id(file_id))
            .await
    }

    pub async fn get_fas_by_user_id(&self, user_id: ObjectId) -> Result<Vec<FileCollaborator>> {
        self.file_collaborator
            .get_many(FileCollaborator::user_id(user_id))
            .await
    }

    pub async fn get_fa(&self, file_id: ObjectId, user_id: ObjectId) -> Result<FileCollaborator> {
        self.file_collaborator
            .get_one(FileCollaborator::file_id(file_id).user_id(user_id))
            .await
    }
}

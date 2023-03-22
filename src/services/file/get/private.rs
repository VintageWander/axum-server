use mongodb::bson::oid::ObjectId;

use crate::{
    model::{file::File, user::User},
    services::Service,
    validation::file::check_fullpath,
    Result,
};

impl Service {
    pub async fn get_files_by_owner(&self, owner: &User) -> Result<Vec<File>> {
        self.file_repo.get_files_by_owner(owner).await
    }

    pub async fn get_file_by_id_owner(&self, file_id: ObjectId, owner: &User) -> Result<File> {
        self.file_repo
            .get_file_by_id_owner(file_id, owner)
            .await
    }

    pub async fn get_file_by_fullpath(&self, fullpath: &str) -> Result<File> {
        check_fullpath(fullpath)?;
        self.file_repo
            .get_file_by_fullpath(fullpath)
            .await
    }

    pub async fn get_files_by_position(&self, position: &str) -> Result<Vec<File>> {
        self.file_repo
            .get_files_by_position(position)
            .await
    }
}

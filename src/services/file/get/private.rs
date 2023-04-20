use mongodb::bson::oid::ObjectId;

use crate::{
    model::{file::File, user::User},
    services::Service,
    validation::file::{check_dir, check_fullpath},
    Result,
};

impl Service {
    pub async fn get_files_by_owner(&self, owner: &User) -> Result<Vec<File>> {
        self.file_dao
            .get_many(File::owner(owner.id))
            .await
    }

    pub async fn get_file_by_id(&self, file_id: ObjectId) -> Result<File> {
        self.file_dao.get_one(File::id(file_id)).await
    }

    pub async fn get_file_by_id_owner(&self, file_id: ObjectId, owner: &User) -> Result<File> {
        self.file_dao
            .get_one(File::id(file_id).owner(owner.id))
            .await
    }

    pub async fn get_file_by_fullpath(&self, fullpath: &str) -> Result<File> {
        check_fullpath(fullpath)?;
        self.file_dao
            .get_one(File::fullpath(fullpath))
            .await
    }

    pub async fn get_files_by_position(&self, position: &str) -> Result<Vec<File>> {
        check_dir(position)?;
        self.file_dao
            .get_many(File::position(position))
            .await
    }
}

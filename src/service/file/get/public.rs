use mongodb::bson::oid::ObjectId;

use crate::{
    model::{
        file::{File, FileVisibility},
        user::User,
    },
    service::Service,
    validation::file::check_dir,
    Result,
};

impl Service {
    pub async fn get_public_files(&self) -> Result<Vec<File>> {
        self.file
            .get_many(File::visibility(FileVisibility::Public))
            .await
    }

    pub async fn get_public_files_by_owner(&self, owner: &User) -> Result<Vec<File>> {
        self.file
            .get_many(File::owner(owner.id).visibility(FileVisibility::Public))
            .await
    }

    pub async fn get_public_files_by_position(&self, position: &str) -> Result<Vec<File>> {
        check_dir(position)?;
        self.file
            .get_many(File::position(position).visibility(FileVisibility::Public))
            .await
    }

    pub async fn get_public_file_by_id(&self, file_id: ObjectId) -> Result<File> {
        self.file
            .get_one(File::id(file_id).visibility(FileVisibility::Public))
            .await
    }
}

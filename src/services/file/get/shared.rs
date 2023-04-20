use crate::{
    model::{
        file::{File, FileVisibility},
        user::User,
    },
    services::Service,
    Result,
};

impl Service {
    pub async fn get_shared_files_by_owner(&self, owner: &User) -> Result<Vec<File>> {
        self.file_dao
            .get_many(File::owner(owner.id).visibility(FileVisibility::Shared))
            .await
    }
}

use crate::{
    model::{file::File, user::User},
    services::file::FileService,
    validation::file::check_dir,
    Result,
};

impl FileService {
    pub async fn get_public_files(&self) -> Result<Vec<File>> {
        self.file_repo.get_public_files().await
    }

    pub async fn get_public_files_by_owner(&self, owner: &User) -> Result<Vec<File>> {
        self.file_repo
            .get_public_files_by_owner(owner)
            .await
    }

    pub async fn get_public_files_by_position(&self, position: &str) -> Result<Vec<File>> {
        check_dir(position)?;
        self.file_repo
            .get_public_files_by_position(position)
            .await
    }
}

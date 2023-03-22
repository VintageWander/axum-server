use crate::{
    model::{file::File, user::User},
    services::Service,
    Result,
};

impl Service {
    pub async fn get_shared_files_by_owner(&self, owner: &User) -> Result<Vec<File>> {
        self.file_repo
            .get_shared_files_by_owner(owner)
            .await
    }
}

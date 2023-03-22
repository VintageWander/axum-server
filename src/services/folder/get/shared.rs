use crate::{
    model::{folder::Folder, user::User},
    services::Service,
    Result,
};

impl Service {
    pub async fn get_shared_folders_by_owner(&self, owner: &User) -> Result<Vec<Folder>> {
        self.folder_repo
            .get_shared_folders_by_owner(owner)
            .await
    }
}

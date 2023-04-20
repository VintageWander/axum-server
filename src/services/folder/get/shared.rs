use crate::{
    model::{
        folder::{Folder, FolderVisibility},
        user::User,
    },
    services::Service,
    Result,
};

impl Service {
    pub async fn get_shared_folders_by_owner(&self, owner: &User) -> Result<Vec<Folder>> {
        self.folder_dao
            .get_many(Folder::visibility(FolderVisibility::Shared).owner(owner.id))
            .await
    }
}

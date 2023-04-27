use crate::{
    model::{
        folder::{Folder, FolderVisibility},
        user::User,
    },
    service::Service,
    Result,
};

impl Service {
    pub async fn get_shared_folders_by_owner(&self, owner: &User) -> Result<Vec<Folder>> {
        self.folder
            .get_many(Folder::visibility(FolderVisibility::Shared).owner(owner.id))
            .await
    }
    // This function gets all accessors from a folder
    pub async fn get_accessors_from_folder(&self, folder: &Folder) -> Result<Vec<User>> {
        let fas = self.get_foas_by_folder_id(folder.id).await?;
        let mut users = vec![];
        for fa in fas {
            let user = self.get_user_by_id(fa.user_id).await?;
            users.push(user);
        }
        Ok(users)
    }

    // This function gets all folders that a user has been shared to
    pub async fn get_shared_folders_from_accessor(&self, accessor: &User) -> Result<Vec<Folder>> {
        let fas = self.get_foas_by_user_id(accessor.id).await?;
        let mut folders = vec![];
        for fa in fas {
            let folder = self.get_folder_by_id(fa.folder_id).await?;
            folders.push(folder);
        }
        Ok(folders)
    }
}

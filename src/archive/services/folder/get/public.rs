use crate::{
    model::{folder::Folder, user::User},
    services::folder::FolderService,
    validation::file::check_dir,
    Result,
};

impl FolderService {
    pub async fn get_public_folders(&self) -> Result<Vec<Folder>> {
        self.folder_repo.get_public_folders().await
    }
    pub async fn get_public_folders_by_owner(&self, owner: &User) -> Result<Vec<Folder>> {
        self.folder_repo
            .get_public_folders_by_owner(owner)
            .await
    }

    pub async fn get_public_folders_by_position(&self, position: &str) -> Result<Vec<Folder>> {
        check_dir(position)?;
        self.folder_repo
            .get_public_folders_by_position(position)
            .await
    }

    pub async fn get_public_folder_by_fullpath(&self, fullpath: &str) -> Result<Folder> {
        check_dir(fullpath)?;
        self.folder_repo
            .get_public_folder_by_fullpath(fullpath)
            .await
    }
}

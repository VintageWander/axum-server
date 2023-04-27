use serde::{Deserialize, Serialize};

use crate::model::{file::*, folder::*, user::*};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPopulated {
    #[serde(flatten)]
    pub user: UserDTO,
    pub public_files: Vec<FileDTO>,
    pub my_shared_files: Vec<FileDTO>,
    pub other_shared_files: Vec<FileDTO>,
    pub private_files: Vec<FileDTO>,
    pub public_folders: Vec<FolderDTO>,
    pub my_shared_folders: Vec<FolderDTO>,
    pub other_shared_folders: Vec<FolderDTO>,
    pub private_folders: Vec<FolderDTO>,
}

#[allow(clippy::too_many_arguments)]
impl UserPopulated {
    pub fn new(
        user: User,
        public_files: Vec<File>,
        my_shared_files: Vec<File>,
        other_shared_files: Vec<File>,
        private_files: Vec<File>,
        public_folders: Vec<Folder>,
        my_shared_folders: Vec<Folder>,
        other_shared_folders: Vec<Folder>,
        private_folders: Vec<Folder>,
    ) -> Self {
        let user_dto = user.into_dto();
        let public_files = public_files
            .into_iter()
            .map(|f| f.into_dto())
            .collect::<Vec<_>>();

        let my_shared_files = my_shared_files
            .into_iter()
            .map(|f| f.into_dto())
            .collect::<Vec<_>>();

        let other_shared_files = other_shared_files
            .into_iter()
            .map(|f| f.into_dto())
            .collect::<Vec<_>>();

        let private_files = private_files
            .into_iter()
            .map(|f| f.into_dto())
            .collect::<Vec<_>>();

        let public_folders = public_folders
            .into_iter()
            .map(|f| f.into_dto())
            .collect::<Vec<_>>();

        let my_shared_folders = my_shared_folders
            .into_iter()
            .map(|f| f.into_dto())
            .collect::<Vec<_>>();

        let other_shared_folders = other_shared_folders
            .into_iter()
            .map(|f| f.into_dto())
            .collect::<Vec<_>>();

        let private_folders = private_folders
            .into_iter()
            .map(|f| f.into_dto())
            .collect::<Vec<_>>();

        Self {
            user: user_dto,
            public_files,
            my_shared_files,
            other_shared_files,
            private_files,
            public_folders,
            my_shared_folders,
            other_shared_folders,
            private_folders,
        }
    }
}

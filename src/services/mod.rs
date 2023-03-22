pub mod file;
pub mod file_version;
pub mod folder;
pub mod user;

use crate::{
    dao::storage::Storage,
    db::{aws::S3, mongo::Mongo},
    repo::{file::FileRepo, file_version::FileVersionRepo, folder::FolderRepo, user::UserRepo},
};

#[derive(Debug, Clone)]
pub struct Service {
    user_repo: UserRepo,
    file_repo: FileRepo,
    folder_repo: FolderRepo,
    file_version_repo: FileVersionRepo,
    pub storage: Storage,
}

impl Service {
    pub fn init(db: &Mongo) -> Self {
        Self {
            user_repo: UserRepo::init(db),
            file_repo: FileRepo::init(db),
            folder_repo: FolderRepo::init(db),
            file_version_repo: FileVersionRepo::init(db),
            storage: Storage::init(&S3::init()),
        }
    }
}

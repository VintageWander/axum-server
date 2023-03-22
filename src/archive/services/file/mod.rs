pub mod create;
pub mod delete;
pub mod exists;
pub mod get;
pub mod replace;
pub mod restore;
pub mod update;

use crate::{
    dao::storage::Storage,
    db::{aws::S3, mongo::Mongo},
    repo::{file::FileRepo, folder::FolderRepo},
};

use super::file_version::FileVersionService;

#[derive(Debug, Clone)]
pub struct FileService {
    file_repo: FileRepo,
    folder_repo: FolderRepo,
    file_version_service: FileVersionService,
    storage: Storage,
}

impl FileService {
    pub fn init(db: &Mongo) -> Self {
        Self {
            file_repo: FileRepo::init(db),
            folder_repo: FolderRepo::init(db),
            file_version_service: FileVersionService::init(db),
            storage: Storage::init(&S3::init()),
        }
    }
}

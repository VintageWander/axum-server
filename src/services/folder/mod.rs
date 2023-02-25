pub mod create;
pub mod delete;
pub mod exists;
pub mod get;
pub mod update;

use crate::{db::mongo::Mongo, repo::folder::FolderRepo};

use super::file::FileService;

#[derive(Debug, Clone)]
pub struct FolderService {
    folder_repo: FolderRepo,
    file_service: FileService,
}

impl FolderService {
    pub fn init(db: &Mongo) -> Self {
        Self {
            folder_repo: FolderRepo::init(db),
            file_service: FileService::init(db),
        }
    }
}

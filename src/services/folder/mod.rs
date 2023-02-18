use crate::{db::mongo::Mongo, repo::folder::FolderRepo};

#[derive(Debug, Clone)]
pub struct FolderService {
    folder_repo: FolderRepo,
}

impl FolderService {
    pub fn init(db: &Mongo) -> Self {
        Self {
            folder_repo: FolderRepo::init(db),
        }
    }
}

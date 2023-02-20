pub mod create;
pub mod delete;
pub mod exists;
pub mod get;
pub mod update;

use crate::{db::mongo::Mongo, repo::user::UserRepo};

use super::folder::FolderService;

#[derive(Debug, Clone)]
pub struct UserService {
    user_repo: UserRepo,
    folder_service: FolderService,
}

impl UserService {
    pub fn init(db: &Mongo) -> Self {
        Self {
            user_repo: UserRepo::init(db),
            folder_service: FolderService::init(db),
        }
    }
}

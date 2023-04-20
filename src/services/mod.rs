pub mod file;
pub mod file_version;
pub mod folder;
pub mod user;

use crate::{
    dao::storage::Storage,
    db::{aws::S3, mongo::Mongo},
    model::{file::*, file_version::*, folder::*, user::*},
};

#[derive(Debug, Clone)]
pub struct Service {
    user_dao: UserDao,
    file_dao: FileDao,
    folder_dao: FolderDao,
    file_version_dao: FileVersionDao,
    pub storage: Storage,
}

impl Service {
    pub fn init(db: &Mongo) -> Self {
        Self {
            user_dao: UserDao::new(db.get_collection("User")),
            file_dao: FileDao::new(db.get_collection("File")),
            folder_dao: FolderDao::new(db.get_collection("Folder")),
            file_version_dao: FileVersionDao::new(db.get_collection("FileVersion")),
            storage: Storage::init(&S3::init()),
        }
    }
}

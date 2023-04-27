pub mod file;
pub mod file_accessor;
pub mod file_version;
pub mod folder;
pub mod folder_accessor;
pub mod user;

use crate::{
    dao::storage::Storage,
    db::{aws::S3, mongo::Mongo},
    model::{file::*, file_accessor::*, file_version::*, folder::*, folder_accessor::*, user::*},
};

#[derive(Debug, Clone)]
pub struct Service {
    user: UserDao,
    file: FileDao,
    folder: FolderDao,
    file_version: FileVersionDao,
    file_accessor: FileAccessorDao,
    folder_accessor: FolderAccessorDao,
    pub storage: Storage,
}

impl Service {
    pub fn init(db: &Mongo) -> Self {
        Self {
            user: UserDao::new(db.get_collection("User")),
            file: FileDao::new(db.get_collection("File")),
            folder: FolderDao::new(db.get_collection("Folder")),
            file_version: FileVersionDao::new(db.get_collection("FileVersion")),
            file_accessor: FileAccessorDao::new(db.get_collection("FileAccessor")),
            folder_accessor: FolderAccessorDao::new(db.get_collection("FolderAccessor")),
            storage: Storage::init(&S3::init()),
        }
    }
}

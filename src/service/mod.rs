pub mod file;
pub mod file_collaborator;
pub mod file_version;
pub mod folder;
pub mod folder_collaborator;
pub mod user;

use crate::{
    dao::storage::Storage,
    db::{aws::S3, mongo::Mongo},
    model::{
        file::*, file_collaborator::*, file_version::*, folder::*, folder_collaborator::*, user::*,
    },
};

#[derive(Debug, Clone)]
pub struct Service {
    user: UserDao,
    file: FileDao,
    folder: FolderDao,
    file_version: FileVersionDao,
    file_collaborator: FileCollaboratorDao,
    folder_collaborator: FolderCollaboratorDao,
    pub storage: Storage,
}

impl Service {
    pub fn init(db: &Mongo) -> Self {
        Self {
            user: UserDao::new(db.get_collection("User")),
            file: FileDao::new(db.get_collection("File")),
            folder: FolderDao::new(db.get_collection("Folder")),
            file_version: FileVersionDao::new(db.get_collection("FileVersion")),
            file_collaborator: FileCollaboratorDao::new(db.get_collection("FileCollaborator")),
            folder_collaborator: FolderCollaboratorDao::new(
                db.get_collection("FolderCollaborator"),
            ),
            storage: Storage::init(&S3::init()),
        }
    }
}

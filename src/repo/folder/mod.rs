use crate::{db::mongo::Mongo, model::folder::*};

pub mod create;
pub mod delete;
pub mod exists;
pub mod get;
pub mod replace;
pub mod update;

#[derive(Debug, Clone)]
pub struct FolderRepo {
    folder_dao: FolderDao,
}

impl FolderRepo {
    pub fn init(db: &Mongo) -> Self {
        Self {
            folder_dao: FolderDao::new(db.get_collection("Folder")),
        }
    }
}

use crate::{dao::Dao, db::mongo::Mongo, model::folder::Folder};

pub mod create;
pub mod delete;
pub mod exists;
pub mod get;
pub mod replace;
pub mod update;

#[derive(Debug, Clone)]
pub struct FolderRepo {
    folder_dao: Dao<Folder>,
}

impl FolderRepo {
    pub fn init(db: &Mongo) -> Self {
        Self {
            folder_dao: Dao::init(db, "Folder"),
        }
    }
}

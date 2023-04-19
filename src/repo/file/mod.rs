pub mod create;
pub mod delete;
pub mod exists;
pub mod get;
pub mod replace;
pub mod update;

use crate::{db::mongo::Mongo, model::file::*};

#[derive(Debug, Clone)]
pub struct FileRepo {
    file_dao: FileDao,
}

impl FileRepo {
    pub fn init(db: &Mongo) -> Self {
        Self {
            file_dao: FileDao::new(db.get_collection("File")),
        }
    }
}

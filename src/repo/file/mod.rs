pub mod create;
pub mod delete;
pub mod exists;
pub mod get;
pub mod replace;
pub mod update;

use crate::{dao::mongo::Dao, db::mongo::Mongo, model::file::File};

#[derive(Debug, Clone)]
pub struct FileRepo {
    file_dao: Dao<File>,
}

impl FileRepo {
    pub fn init(db: &Mongo) -> Self {
        Self {
            file_dao: Dao::init(db, "File"),
        }
    }
}

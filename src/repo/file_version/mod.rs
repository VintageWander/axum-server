use crate::{db::mongo::Mongo, model::file_version::*};

pub mod create;
pub mod delete;
pub mod get;

// This repository does not have a service, since I do not have any intent to expose it outside
// Normally services are created for the handlers to use
// This repository only be used internally

#[derive(Debug, Clone)]
pub struct FileVersionRepo {
    file_version_dao: FileVersionDao,
}

impl FileVersionRepo {
    pub fn init(db: &Mongo) -> Self {
        Self {
            file_version_dao: FileVersionDao::new(db.get_collection("FileVersion")),
        }
    }
}

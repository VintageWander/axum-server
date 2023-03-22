use crate::{
    dao::storage::Storage,
    db::{aws::S3, mongo::Mongo},
    repo::file_version::FileVersionRepo,
};

pub mod create;
pub mod delete;
pub mod get;

#[derive(Debug, Clone)]
pub struct FileVersionService {
    file_version_repo: FileVersionRepo,
    storage: Storage,
}

impl FileVersionService {
    pub fn init(db: &Mongo) -> Self {
        Self {
            file_version_repo: FileVersionRepo::init(db),
            storage: Storage::init(&S3::init()),
        }
    }
}

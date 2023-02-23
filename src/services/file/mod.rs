pub mod create;

use crate::{
    dao::s3::Storage,
    db::{aws::S3, mongo::Mongo},
    repo::file::FileRepo,
};

#[derive(Debug, Clone)]
pub struct FileService {
    file_repo: FileRepo,
    storage: Storage,
}

impl FileService {
    pub fn init(db: &Mongo, s3: &S3) -> Self {
        Self {
            file_repo: FileRepo::init(db),
            storage: Storage::init(s3),
        }
    }
}

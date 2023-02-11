pub mod create;
pub mod delete;
pub mod exists;
pub mod get;
pub mod update;

use crate::{db::mongo::Mongo, repo::user::UserRepo};

#[derive(Debug, Clone)]
pub struct UserService {
    user_repo: UserRepo,
}

impl UserService {
    pub fn init(db: &Mongo) -> Self {
        Self {
            user_repo: UserRepo::init(db),
        }
    }
}

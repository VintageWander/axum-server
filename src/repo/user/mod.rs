pub mod create;
pub mod delete;
pub mod exists;
pub mod get;
pub mod update;

use crate::{db::mongo::Mongo, model::user::*};

#[derive(Debug, Clone)]
pub struct UserRepo {
    user_dao: UserDao,
}

impl UserRepo {
    pub fn init(db: &Mongo) -> Self {
        Self {
            user_dao: UserDao::new(db.get_collection("User")),
        }
    }
}

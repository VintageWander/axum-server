pub mod create;
pub mod delete;
pub mod exists;
pub mod get;
pub mod update;

use crate::{dao::Dao, db::mongo::Mongo, model::user::User};

#[derive(Debug, Clone)]
pub struct UserRepo {
    user_dao: Dao<User>,
}

impl UserRepo {
    pub fn init(db: &Mongo) -> Self {
        Self {
            user_dao: Dao::<User>::init(db, "User"),
        }
    }
}

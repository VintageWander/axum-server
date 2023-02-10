pub mod create;
pub mod delete;
pub mod exists;
pub mod get;
pub mod update;

use crate::{dao::Dao, model::user::User};

#[derive(Debug, Clone)]
pub struct UserRepo {
    user_dao: Dao<User>,
}

impl UserRepo {
    pub fn init(user_dao: &Dao<User>) -> Self {
        Self {
            user_dao: user_dao.clone(),
        }
    }
}

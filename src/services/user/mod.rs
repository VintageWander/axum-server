pub mod create;
pub mod delete;
pub mod exists;
pub mod get;
pub mod update;

use crate::repo::user::UserRepo;

#[derive(Debug, Clone)]
pub struct UserService {
    user_repo: UserRepo,
}

impl UserService {
    pub fn init(user_repo: &UserRepo) -> Self {
        Self {
            user_repo: user_repo.clone(),
        }
    }
}

use crate::model::{file::*, user::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilePopulated {
    #[serde(flatten)]
    pub file: FileDTO,
    pub owner: UserDTO,
}

impl FilePopulated {
    pub fn new(file: File, owner: User) -> Self {
        Self {
            file: file.into_dto(),
            owner: owner.into_dto(),
        }
    }
}

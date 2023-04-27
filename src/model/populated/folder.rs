use crate::model::{folder::*, user::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderPopulated {
    pub folder: FolderDTO,
    #[serde(flatten)]
    pub owner: UserDTO,
}

impl FolderPopulated {
    pub fn new(folder: Folder, owner: User) -> Self {
        Self {
            folder: folder.into_dto(),
            owner: owner.into_dto(),
        }
    }
}

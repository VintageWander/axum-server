use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{model::folder::Folder, validation::file::*};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FolderResponse {
    #[serde(rename = "_id")]
    pub id: String,
    pub owner: String,

    #[validate(custom = "check_folder_name")]
    pub folder_name: String,

    #[validate(custom = "check_dir")]
    pub position: String,

    pub visibility: String,

    #[validate(custom = "check_dir")]
    pub fullpath: String,

    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Folder> for FolderResponse {
    fn from(f: Folder) -> Self {
        Self {
            visibility: f.visibility_to_str().to_string(),
            id: f.id.to_string(),
            owner: f.owner.to_string(),
            folder_name: f.folder_name,
            position: f.position,
            fullpath: f.fullpath,
            created_at: f.created_at,
            updated_at: f.updated_at,
        }
    }
}

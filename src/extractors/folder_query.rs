use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::validation::file::*;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FolderQuery {
    pub id: Option<String>,
    pub owner: Option<String>,

    #[validate(custom = "check_folder_name")]
    pub folder_name: Option<String>,

    #[validate(custom = "check_dir")]
    pub position: Option<String>,

    pub visibility: Option<String>,

    #[validate(custom = "check_dir")]
    pub fullpath: Option<String>,

    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

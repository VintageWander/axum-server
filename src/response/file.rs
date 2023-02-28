use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{model::file::File, validation::file::*};

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FileResponse {
    #[serde(rename = "_id")]
    pub id: String,

    pub owner: String,

    #[validate(custom = "check_filename")]
    pub filename: String,

    pub extension: String,

    #[validate(custom = "check_full_filename")]
    pub full_filename: String,

    pub visibility: String,

    #[validate(custom = "check_dir")]
    pub position: String,

    #[validate(custom = "check_fullpath")]
    pub fullpath: String,

    pub created_at: i64,
    pub updated_at: i64,
}

impl From<File> for FileResponse {
    fn from(f: File) -> Self {
        Self {
            id: f.id.to_string(),
            owner: f.owner.to_string(),
            filename: f.filename,
            extension: f.extension.into(),
            full_filename: f.full_filename,
            visibility: f.visibility.into(),
            position: f.position,
            fullpath: f.fullpath,
            created_at: f.created_at,
            updated_at: f.updated_at,
        }
    }
}

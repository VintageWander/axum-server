use backend_macros::Dto;
use mongodb::bson::{doc, oid::ObjectId};
use mongoose::Model;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::file::File;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Model, Dto)]
#[serde(rename_all = "camelCase")]
pub struct FileVersion {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub file_id: ObjectId,
    pub version_number: i64,
}

impl FileVersion {
    pub fn new(file: &File, version_number: i64) -> Self {
        Self {
            id: ObjectId::new(),
            file_id: file.id,
            version_number,
        }
    }
}

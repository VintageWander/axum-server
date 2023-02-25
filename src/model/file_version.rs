use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

use super::file::File;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileVersion {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub file_id: ObjectId,
    pub version_number: i64,
}

impl From<FileVersion> for Document {
    fn from(fv: FileVersion) -> Self {
        doc! {
            "fileId": fv.file_id,
            "versionNumber": fv.version_number
        }
    }
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

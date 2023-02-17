use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{error::Error, helper::make_error::validation_message, validation::file::*, Result};

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub owner: ObjectId,

    #[validate(custom = "check_folder_name")]
    pub folder_name: String,

    #[validate(custom = "check_dir")]
    pub position: String,

    pub visibility: FolderVisibility,

    #[validate(custom = "check_dir")]
    pub fullpath: String,

    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FolderVisibility {
    Public,
    Shared,
    Private,
}

impl Folder {
    pub fn visibility_to_str(&self) -> &str {
        match self.visibility {
            FolderVisibility::Public => "public",
            FolderVisibility::Shared => "shared",
            FolderVisibility::Private => "private",
        }
    }
    pub fn str_to_visibility(visibility: &str) -> Result<FolderVisibility> {
        Ok(match visibility {
            "public" => FolderVisibility::Public,
            "shared" => FolderVisibility::Shared,
            "private" => FolderVisibility::Private,
            _ => return Err(Error::Field(validation_message("Invalid visibility type"))),
        })
    }
}

use backend_macros::{Dto, IntoSearchQuery};
use chrono::Utc;
use mongodb::bson::{doc, oid::ObjectId};
use mongoose::{IntoDoc, Model};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{error::Error, helper::make_error::validation_message, validation::file::*, Result};

use super::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Model, Dto, IntoSearchQuery)]
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

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, IntoDoc)]
#[serde(rename_all = "lowercase")]
pub enum FolderVisibility {
    #[default]
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "shared")]
    Shared,
    #[serde(rename = "private")]
    Private,
}

impl TryFrom<&str> for FolderVisibility {
    type Error = Error;
    fn try_from(visibility: &str) -> std::result::Result<Self, Self::Error> {
        Ok(match visibility {
            "public" => FolderVisibility::Public,
            "shared" => FolderVisibility::Shared,
            "private" => FolderVisibility::Private,
            _ => return Err(Error::Field(validation_message("Invalid visibility type"))),
        })
    }
}

impl TryFrom<String> for FolderVisibility {
    type Error = Error;
    fn try_from(visibility: String) -> std::result::Result<Self, Self::Error> {
        visibility.as_str().try_into()
    }
}

impl Folder {
    pub fn new(
        id: ObjectId,
        owner: &User,
        folder_name: String,
        position: String,
        visibility: FolderVisibility,
        created_at: i64,
    ) -> Result<Self> {
        check_folder_name(&folder_name)?;
        check_dir(&position)?;

        // folder_name = something
        // position = folder/

        let position_with_owner = format!("{}/{}", owner.username, position);
        // position_with_owner = User/folder/

        let fullpath = format!("{}{}/", position_with_owner, folder_name);
        // fullpath = User/folder/something/

        let folder = Folder {
            id,
            owner: owner.id,
            folder_name,
            position: position_with_owner,
            visibility,
            fullpath,
            created_at,
            updated_at: Utc::now().timestamp_millis(),
        };

        folder.validate()?;

        Ok(folder)
    }

    pub fn new_root(owner: &User) -> Result<Self> {
        let root_folder = Folder {
            id: ObjectId::new(),
            owner: owner.id,
            folder_name: owner.username.clone(),
            position: format!("{}/", owner.username),
            visibility: FolderVisibility::Private,
            fullpath: format!("{}/", owner.username),
            created_at: Utc::now().timestamp_millis(),
            updated_at: Utc::now().timestamp_millis(),
        };

        root_folder.validate()?;

        Ok(root_folder)
    }

    pub fn into_response(self) -> FolderDTO {
        self.into_dto()
    }
}

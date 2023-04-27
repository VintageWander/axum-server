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
pub struct File {
    #[serde(rename = "_id")]
    pub id: ObjectId,

    pub owner: ObjectId,

    #[validate(custom = "check_filename")]
    pub filename: String,

    pub extension: FileExtension,

    #[validate(custom = "check_full_filename")]
    pub full_filename: String,

    pub visibility: FileVisibility,

    #[validate(custom = "check_dir")]
    pub position: String,

    #[validate(custom = "check_fullpath")]
    pub fullpath: String,

    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, IntoDoc)]
#[serde(rename = "lowercase")]
pub enum FileExtension {
    #[default]
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "jpg")]
    Jpg,
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "mp3")]
    Mp3,
    #[serde(rename = "txt")]
    Txt,
}

impl TryFrom<&str> for FileExtension {
    type Error = Error;
    fn try_from(str: &str) -> std::result::Result<Self, Self::Error> {
        Ok(match str {
            "png" => FileExtension::Png,
            "jpg" => FileExtension::Jpg,
            "jpeg" => FileExtension::Jpeg,
            "mp3" => FileExtension::Mp3,
            "txt" => FileExtension::Txt,
            _ => return Err(Error::Field(validation_message("Unsupported extension"))),
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, IntoDoc)]
#[serde(rename = "lowercase")]
pub enum FileVisibility {
    #[default]
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "shared")]
    Shared,
    #[serde(rename = "private")]
    Private,
}

impl TryFrom<String> for FileVisibility {
    type Error = Error;
    fn try_from(str: String) -> std::result::Result<Self, Self::Error> {
        Ok(match str.as_str() {
            "public" => FileVisibility::Public,
            "private" => FileVisibility::Private,
            "shared" => FileVisibility::Shared,
            _ => return Err(Error::Field(validation_message("Invalid visibility type"))),
        })
    }
}

impl File {
    pub fn new(
        id: ObjectId,
        owner: &User,
        full_filename: String,
        visibility: FileVisibility,
        position: String,
        created_at: i64,
    ) -> Result<Self> {
        check_full_filename(full_filename.as_ref())?;
        // full_filename = hello.txt
        // position = folder/
        // owner.username = User

        let (filename, extension) = full_filename
            .rsplit_once('.')
            .ok_or(Error::Split)?;
        // filename = hello
        // extension = txt

        let extension: FileExtension = extension.try_into()?;
        // extension = FileExtension::Txt,

        let position_with_owner = format!("{}/{}", owner.username, position);
        // position_with_owner = User/folder/

        let fullpath = format!("{}{}", position_with_owner, full_filename);
        // fullpath = User/folder/hello.txt

        let file = Self {
            id,
            owner: owner.id,
            filename: filename.into(),
            extension,
            full_filename,
            visibility,
            position: position_with_owner,
            fullpath,
            created_at,
            updated_at: Utc::now().timestamp_millis(),
        };

        file.validate()?;
        Ok(file)
    }

    pub fn into_response(self) -> FileDTO {
        self.into_dto()
    }
}

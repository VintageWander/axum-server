use chrono::Utc;
use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{error::Error, helper::make_error::validation_message, validation::file::*, Result};

use super::user::User;

#[derive(Debug, Serialize, Deserialize, Validate)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "lowercase")]
pub enum FileExtension {
    Png,
    Jpg,
    Jpeg,
    Mp3,
    Txt,
}

impl From<FileExtension> for &str {
    fn from(f: FileExtension) -> Self {
        match f {
            FileExtension::Png => "png",
            FileExtension::Jpg => "jpg",
            FileExtension::Jpeg => "jpeg",
            FileExtension::Mp3 => "mp3",
            FileExtension::Txt => "txt",
        }
    }
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "lowercase")]
pub enum FileVisibility {
    Public,
    Shared,
    Private,
}

impl Default for FileVisibility {
    fn default() -> Self {
        FileVisibility::Public
    }
}

impl From<FileVisibility> for &str {
    fn from(f: FileVisibility) -> Self {
        match f {
            FileVisibility::Public => "public",
            FileVisibility::Shared => "shared",
            FileVisibility::Private => "private",
        }
    }
}

impl TryFrom<&str> for FileVisibility {
    type Error = Error;
    fn try_from(str: &str) -> std::result::Result<Self, Self::Error> {
        Ok(match str {
            "public" => FileVisibility::Public,
            "private" => FileVisibility::Private,
            "shared" => FileVisibility::Shared,
            _ => return Err(Error::Field(validation_message("Invalid visibility type"))),
        })
    }
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

impl From<File> for Document {
    fn from(f: File) -> Self {
        let extension: &str = f.extension.into();
        let visibility: &str = f.visibility.into();

        doc! {
            "owner": f.owner,
            "filename": f.filename,
            "extension": extension,
            "fullFilename": f.full_filename,
            "visibility": visibility,
            "position": f.position,
            "fullpath": f.fullpath,
            "createdAt": f.created_at,
            "updatedAt": f.updated_at,
        }
    }
}

impl File {
    pub fn new(
        id: &ObjectId,
        owner: &User,
        full_filename: &str,
        visibility: FileVisibility,
        position: &str,
        created_at: i64,
    ) -> Result<Self> {
        check_full_filename(full_filename)?;
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
            id: *id,
            owner: owner.id,
            filename: filename.into(),
            extension,
            full_filename: full_filename.into(),
            visibility,
            position: position_with_owner,
            fullpath,
            created_at,
            updated_at: Utc::now().timestamp_millis(),
        };

        file.validate()?;

        Ok(file)
    }
}

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "lowercase")]
pub enum FileVisibility {
    Public,
    Shared,
    Private,
}

impl From<File> for Document {
    fn from(f: File) -> Self {
        let extension = f.extension_to_str();
        let visibility = f.visibility_to_str();

        doc! {
            "extension": extension,
            "visibility": visibility,
            "owner": f.owner,
            "filename": f.filename,
            "fullFilename": f.full_filename,
            "position": f.position,
            "fullpath": f.fullpath,
            "createdAt": f.created_at,
            "updatedAt": f.updated_at,
        }
    }
}

impl File {
    pub fn extension_to_str(&self) -> &str {
        match self.extension {
            FileExtension::Png => "png",
            FileExtension::Jpg => "jpg",
            FileExtension::Jpeg => "jpeg",
            FileExtension::Mp3 => "mp3",
            FileExtension::Txt => "txt",
        }
    }

    pub fn str_to_extension(extension: &str) -> Result<FileExtension> {
        Ok(match extension {
            "png" => FileExtension::Png,
            "jpg" => FileExtension::Jpg,
            "jpeg" => FileExtension::Jpeg,
            "mp3" => FileExtension::Mp3,
            "txt" => FileExtension::Txt,
            _ => return Err(Error::Field(validation_message("Unsupported extension"))),
        })
    }

    pub fn visibility_to_str(&self) -> &str {
        match self.visibility {
            FileVisibility::Public => "public",
            FileVisibility::Shared => "shared",
            FileVisibility::Private => "private",
        }
    }

    pub fn str_to_visibility(visibility: &str) -> Result<FileVisibility> {
        Ok(match visibility {
            "public" => FileVisibility::Public,
            "private" => FileVisibility::Private,
            "shared" => FileVisibility::Shared,
            _ => return Err(Error::Field(validation_message("Invalid visibility type"))),
        })
    }

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

        let (filename, extension) = full_filename.rsplit_once('.').ok_or(Error::Split)?;
        // filename = hello
        // extension = txt

        let extension = File::str_to_extension(extension)?;
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

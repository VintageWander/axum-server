use chrono::Utc;
use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    error::Error, helper::make_error::validation_message, response::folder::FolderResponse,
    validation::file::*, Result,
};

use super::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FolderVisibility {
    Public,
    Shared,
    Private,
}

impl TryFrom<String> for FolderVisibility {
    type Error = Error;
    fn try_from(visibility: String) -> std::result::Result<Self, Self::Error> {
        Ok(match visibility.as_str() {
            "public" => FolderVisibility::Public,
            "shared" => FolderVisibility::Shared,
            "private" => FolderVisibility::Private,
            _ => return Err(Error::Field(validation_message("Invalid visibility type"))),
        })
    }
}

impl From<Folder> for Document {
    fn from(f: Folder) -> Self {
        let visibility = f.visibility_to_str();

        doc! {
            "visibilty": visibility,
            "owner": f.owner,
            "folderName": f.folder_name,
            "position": f.position,
            "fullpath": f.fullpath,
            "createAt": f.created_at,
            "updatedAt": f.updated_at,
        }
    }
}

impl Folder {
    pub fn visibility_to_str(&self) -> &str {
        match self.visibility {
            FolderVisibility::Public => "public",
            FolderVisibility::Shared => "shared",
            FolderVisibility::Private => "private",
        }
    }

    pub fn new(
        id: &ObjectId,
        owner: &User,
        folder_name: &str,
        position: &str,
        visibility: FolderVisibility,
        created_at: i64,
    ) -> Result<Self> {
        check_folder_name(folder_name)?;
        check_dir(position)?;

        // folder_name = something
        // position = folder/

        let position_with_owner = format!("{}/{}", owner.username, position);
        // position_with_owner = User/folder/

        let fullpath = format!("{}{}/", position_with_owner, folder_name);
        // fullpath = User/folder/something/

        let folder = Folder {
            id: *id,
            owner: owner.id,
            folder_name: folder_name.into(),
            position: position_with_owner,
            visibility,
            fullpath,
            created_at,
            updated_at: Utc::now().timestamp_millis(),
        };

        folder.validate()?;

        todo!()
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

    pub fn into_response(self) -> FolderResponse {
        self.into()
    }
}

use backend_macros::Dto;
use mongodb::bson::oid::ObjectId;
use mongoose::Model;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Model, Dto)]
#[serde(rename_all = "camelCase")]
pub struct FolderAccessor {
    #[serde(rename = "_id")]
    pub id: ObjectId,

    pub user_id: ObjectId,
    pub folder_id: ObjectId,

    pub created_at: i64,
    pub updated_at: i64,
}

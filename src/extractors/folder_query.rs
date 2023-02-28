use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderQuery {
    pub id: Option<String>,
    pub owner: Option<String>,

    pub folder_name: Option<String>,

    pub position: Option<String>,

    pub visibility: Option<String>,

    pub fullpath: Option<String>,

    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

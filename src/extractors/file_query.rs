use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileQuery {
    pub id: Option<String>,

    pub owner: Option<String>,

    pub filename: Option<String>,

    pub extension: Option<String>,

    pub full_filename: Option<String>,

    pub visibility: Option<String>,

    pub position: Option<String>,

    pub fullpath: Option<String>,

    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

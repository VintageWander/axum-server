use mongodb::bson::doc;

use crate::{
    model::{file::File, user::User},
    repo::file::FileRepo,
    Result,
};

impl FileRepo {
    pub async fn get_public_files(&self) -> Result<Vec<File>> {
        self.file_dao
            .get_multiple(doc! {"visibility": "public"})
            .await
    }
    pub async fn get_public_files_by_owner(&self, owner: &User) -> Result<Vec<File>> {
        self.file_dao
            .get_multiple(doc! {"owner": owner.id, "visibility": "public"})
            .await
    }
    pub async fn get_public_files_by_position(&self, position: &str) -> Result<Vec<File>> {
        self.file_dao
            .get_multiple(doc! {"visibility": "public", "position": position})
            .await
    }
}

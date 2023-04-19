use mongodb::bson::{doc, oid::ObjectId};

use crate::{
    model::{file::File, user::User},
    repo::file::FileRepo,
    Result,
};

impl FileRepo {
    pub async fn get_public_files(&self) -> Result<Vec<File>> {
        self.file_dao
            .get_many(doc! {"visibility": "public"})
            .await
    }
    pub async fn get_public_files_by_owner(&self, owner: &User) -> Result<Vec<File>> {
        self.file_dao
            .get_many(doc! {"owner": owner.id, "visibility": "public"})
            .await
    }
    pub async fn get_public_files_by_position(&self, position: &str) -> Result<Vec<File>> {
        self.file_dao
            .get_many(doc! {"visibility": "public", "position": position})
            .await
    }

    pub async fn get_public_file_by_id(&self, file_id: ObjectId) -> Result<File> {
        self.file_dao
            .get_one(doc! {"_id": file_id ,"visibility": "public"})
            .await
    }
}

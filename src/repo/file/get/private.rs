use mongodb::bson::{doc, oid::ObjectId, Document};

use crate::{
    model::{file::File, user::User},
    repo::file::FileRepo,
    Result,
};

impl FileRepo {
    pub async fn get_files_by(&self, doc: Document) -> Result<Vec<File>> {
        self.file_dao.get_many(doc).await
    }

    pub async fn get_files_by_owner(&self, owner: &User) -> Result<Vec<File>> {
        self.file_dao
            .get_many(doc! {"owner": owner.id})
            .await
    }

    pub async fn get_file_by_id(&self, file_id: ObjectId) -> Result<File> {
        self.file_dao.get_one(doc! {"_id": file_id}).await
    }

    pub async fn get_file_by_id_owner(&self, file_id: ObjectId, owner: &User) -> Result<File> {
        self.file_dao
            .get_one(doc! {"_id": file_id, "owner": owner.id})
            .await
    }

    pub async fn get_file_by_fullpath(&self, fullpath: &str) -> Result<File> {
        self.file_dao
            .get_one(doc! {"fullpath": fullpath})
            .await
    }

    pub async fn get_files_by_position(&self, position: &str) -> Result<Vec<File>> {
        self.file_dao
            .get_many(doc! {"position": position})
            .await
    }
}

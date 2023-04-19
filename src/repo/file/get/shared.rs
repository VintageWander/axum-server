use mongodb::bson::doc;

use crate::{
    model::{file::File, user::User},
    repo::file::FileRepo,
    Result,
};

impl FileRepo {
    pub async fn get_shared_files_by_owner(&self, owner: &User) -> Result<Vec<File>> {
        self.file_dao
            .get_many(doc! {"visibility": "shared", "owner": owner.id})
            .await
    }
}

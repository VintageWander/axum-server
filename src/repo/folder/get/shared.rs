use mongodb::bson::doc;

use crate::{
    model::{folder::Folder, user::User},
    repo::folder::FolderRepo,
    Result,
};

impl FolderRepo {
    pub async fn get_shared_folders_by_owner(&self, owner: &User) -> Result<Vec<Folder>> {
        self.folder_dao
            .get_many(doc! {"visibility": "shared", "owner": owner.id})
            .await
    }
}

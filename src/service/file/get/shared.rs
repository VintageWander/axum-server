use mongodb::bson::oid::ObjectId;

use crate::{
    model::{
        file::{File, FileVisibility},
        user::User,
    },
    service::Service,
    Result,
};

impl Service {
    pub async fn get_shared_files_by_owner(&self, owner: &User) -> Result<Vec<File>> {
        self.file
            .get_many(File::owner(owner.id).visibility(FileVisibility::Shared))
            .await
    }

    pub async fn get_shared_file_from_collaborator(
        &self,
        file_id: ObjectId,
        collaborator: &User,
    ) -> Result<File> {
        let fa = self.get_fa(file_id, collaborator.id).await?;
        self.get_file_by_id(fa.file_id).await
    }

    pub async fn get_collaborator_from_shared_file(
        &self,
        file: &File,
        collaborator_id: ObjectId,
    ) -> Result<User> {
        let fa = self.get_fa(file.id, collaborator_id).await?;
        self.get_user_by_id(fa.file_id).await
    }

    // This function gets all collaborators from a file
    pub async fn get_collaborators_from_shared_file(&self, file: &File) -> Result<Vec<User>> {
        let fas = self.get_fas_by_file_id(file.id).await?;
        let mut users = vec![];
        for fa in fas {
            let user = self.get_user_by_id(fa.user_id).await?;
            users.push(user);
        }
        Ok(users)
    }

    // This function gets all files that a user has been shared to
    pub async fn get_shared_files_from_collaborator(
        &self,
        collaborator: &User,
    ) -> Result<Vec<File>> {
        let fas = self.get_fas_by_user_id(collaborator.id).await?;
        let mut files = vec![];
        for fa in fas {
            let file = self.get_file_by_id(fa.file_id).await?;
            files.push(file);
        }
        Ok(files)
    }
}

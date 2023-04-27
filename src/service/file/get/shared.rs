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

    // This function gets all accessors from a file
    pub async fn get_accessors_from_file(&self, file: &File) -> Result<Vec<User>> {
        let fas = self.get_fas_by_file_id(file.id).await?;
        let mut users = vec![];
        for fa in fas {
            let user = self.get_user_by_id(fa.user_id).await?;
            users.push(user);
        }
        Ok(users)
    }

    // This function gets all files that a user has been shared to
    pub async fn get_shared_files_from_accessor(&self, accessor: &User) -> Result<Vec<File>> {
        let fas = self.get_fas_by_user_id(accessor.id).await?;
        let mut files = vec![];
        for fa in fas {
            let file = self.get_file_by_id(fa.file_id).await?;
            files.push(file);
        }
        Ok(files)
    }
}

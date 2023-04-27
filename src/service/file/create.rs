use tokio::try_join;

use crate::{
    error::Error,
    model::{file::File, user::User},
    service::Service,
    Result,
};

impl Service {
    pub async fn create_file(&self, file: File, bytes: Vec<u8>) -> Result<File> {
        let (is_duplicate, parent_folder_exists) = try_join!(
            // check for a file with the same name at the exact location
            self.exists_file_by_fullpath(&file.fullpath),
            self.exists_folder_by_fullpath(&file.position), // check in the folder service if there is a folder exists at file's location
        )?;

        if is_duplicate {
            return Err(Error::ConflictFile);
        }

        if !parent_folder_exists {
            return Err(Error::ParentFolderNotFound);
        }

        let file_id = file.id.to_string();
        let full_filename = format!("{}.{}", file_id, &file.extension.to_string());
        let new_file = self.file.insert_one(file).await?;

        try_join!(
            self.storage.put_object(full_filename, bytes),
            self.storage.put_folder(file_id)
        )?;
        Ok(new_file)
    }

    pub async fn add_collaborator_to_file(&self, collaborator: &User, file: &File) -> Result<()> {
        self.link_file_collaborator(file.id, collaborator.id)
            .await?;
        Ok(())
    }
}

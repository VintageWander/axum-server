use tokio::try_join;

use crate::{
    error::Error,
    model::file::{File, FileVisibility},
    service::Service,
    Result,
};

impl Service {
    pub async fn update_file(&self, file: File, bytes: Vec<u8>) -> Result<File> {
        let old_file = self.get_file_by_id(file.id).await?;
        if old_file.extension != file.extension {
            return Err(Error::ExtensionDiff);
        }

        if old_file.fullpath != file.fullpath {
            let (is_duplicate, parent_folder_exists) = try_join!(
                self.exists_file_by_fullpath(&file.fullpath), // check for a file with the same name at the exact location
                self.exists_file_by_fullpath(&file.position) // check in the folder service if there is a folder exists at file's location
            )?;

            if is_duplicate {
                return Err(Error::ConflictFile);
            }

            if !parent_folder_exists {
                return Err(Error::ParentFolderNotFound);
            }
        }

        if !bytes.is_empty() {
            let internal_file_path = &format!("{}.{}", file.id, file.extension.to_string());

            // This function will move the current file to a new place
            // which is at <file_id>/<version_number>.<file_extension>
            self.create_version(&file).await?;
            // This is to put the new file into the storage
            self.storage
                .put_object(internal_file_path, bytes)
                .await?;
        }

        if file.visibility == FileVisibility::Private {
            self.unlink_accessors_from_file(file.id).await?;
        }

        // Update the file model
        self.file.update_one(file).await
    }
}

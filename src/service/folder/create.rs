use tokio::try_join;

use crate::{
    error::Error,
    model::{folder::Folder, user::User},
    service::Service,
    Result,
};

impl Service {
    pub async fn create_folder(&self, folder: Folder) -> Result<Folder> {
        /*
            First we need to check for conflicts by using the exists_folder_by_fullpath
            (Because the fullpath field obviously must be unique)

            Then we need to check if the provided position actually exists
            So that a new folder can be created as a child to some parent folder at that position
            (
                    No worries about creating a folder with no parent,
                    because when the customer creates a new user,
                    a root folder will be created
                    Therefore any folder that getting created with no position provided
                    will be living in the root folder
            )
        */

        let (is_folder_conflict, exists_parent_folder) = try_join!(
            self.exists_folder_by_fullpath(&folder.fullpath), // Check for conflicts
            self.exists_folder_by_fullpath(&folder.position), // Check for parent folder existence
        )?;

        /*
           Why use tokio::try_join! macro?
           Since checking for existence of different things.
           It is better to split these operations into different threads and run them in parallel
        */

        if is_folder_conflict {
            return Err(Error::ConflictFolder);
        }
        if !exists_parent_folder {
            return Err(Error::ParentFolderNotFound);
        }

        let new_folder = self.folder.insert_one(folder).await?;
        Ok(new_folder)
    }

    pub async fn create_root_folder(&self, owner: &User) -> Result<()> {
        self.folder
            .insert_one(Folder::new_root(owner)?)
            .await?;
        Ok(())
    }

    pub async fn add_collaborator_to_folder(
        &self,
        collaborator: &User,
        folder: &Folder,
    ) -> Result<()> {
        self.link_folder_collaborator(folder.id, collaborator.id)
            .await?;
        Ok(())
    }
}

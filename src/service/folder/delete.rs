use futures_util::future::try_join_all;
use mongodb::bson::{doc, oid::ObjectId, Regex};

use crate::{
    model::{folder::Folder, user::User},
    service::Service,
    validation::file::check_dir,
    Result,
};

impl Service {
    pub async fn delete_folder_by_id(&self, folder_id: ObjectId) -> Result<Folder> {
        self.folder
            .delete_one(Folder::id(folder_id))
            .await
    }

    async fn delete_folders_by_prefix_fullpath(&self, fullpath: &str) -> Result<()> {
        check_dir(fullpath)?;
        let fullpath_regex = Regex {
            pattern: format!("^{fullpath}"),
            options: String::new(),
        };
        self.folder
            .delete_many(doc! {"fullpath": {
                    "$regex": fullpath_regex
                }
            })
            .await
    }

    pub async fn delete_folder(&self, folder: Folder) -> Result<()> {
        // Delete the main folder
        let deleted_folder = self.delete_folder_by_id(folder.id).await?;

        // Get all of the child folders
        // I mean ALL of them, since we're doing a regex search
        let child_folders = self
            .get_folders_by_prefix_fullpath(&deleted_folder.fullpath)
            .await?;

        // Create a future vec
        let mut tasks = vec![];

        // Interate through them
        for folder in child_folders {
            tasks.push(self.delete_files_by_folder(folder))
        }

        try_join_all(tasks).await?;

        self.delete_folders_by_prefix_fullpath(&deleted_folder.fullpath)
            .await?;

        Ok(())
    }

    pub async fn delete_root_folder(&self, owner: &User) -> Result<()> {
        // Get the root_folder
        let root_folder = self.get_root_folder(owner).await?;

        // This calls the delete function above, to actually clean up things,
        // both the child folders, child files, file versions, and the folder table
        self.delete_folder(root_folder).await?;

        Ok(())
    }

    pub async fn remove_collaborator_from_folder(
        &self,
        collaborator: &User,
        folder: &Folder,
    ) -> Result<()> {
        self.unlink_folder_collaborator(folder.id, collaborator.id)
            .await?;
        Ok(())
    }
}

use mongodb::bson::{doc, Regex};

use crate::Result;

use super::FolderRepo;

impl FolderRepo {
    pub async fn change_inner_folders_position(
        &self,
        search: &str, // Query all folders that has this search string in its fullpath
        from: &str,   // changing from this string
        to: &str,     // into this string
    ) -> Result<()> {
        let regex = Regex {
            pattern: format!("^{search}"),
            options: String::new(),
        };

        self.folder_dao
            .update_multiple(
                // Find all folders that has the search string in its fullpath
                // If we don't use regex, then the result will only match for one document
                // Since the fullpath is unique
                // We have to use regex to search
                doc! {
                    "fullpath": {
                        "$regex": regex,
                    }
                },
                // Find the "from", and replace them all with "to"
                // This only replaces the section that matches the "from", and replace it
                // Not the entire string itself
                doc! {
                    "$set": {
                        "position": {
                            "$replaceAll": {
                                "input": "$position",
                                "find": from,
                                "replacement": to,
                            }
                        },
                        "fullpath": {
                            "$replaceAll": {
                                "input": "$fullpath",
                                "find": from,
                                "replacement": to,
                            }
                        }
                    }
                },
            )
            .await
    }
}

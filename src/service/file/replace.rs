use mongodb::bson::{doc, Regex};

use crate::{service::Service, Result};

impl Service {
    // This function only exists in the repository layer,
    // It means that only the service layer can use this function
    // I do not want to expose this function on the service layer because it's only purpose
    // is to update the files position, and it is only used on the service layer,
    // specifically in the update method
    pub async fn change_inner_files_position(
        &self,
        search: &str, // Query all files that has this search string in its fullpath
        from: &str,   // find this string inside of their fullpath
        to: &str,     // and change them all into this string
    ) -> Result<()> {
        let regex = Regex {
            pattern: format!("^{search}"),
            options: String::new(),
        };

        self.file
            .collection
            .update_many(
                // Find all files that has the search string in its fullpath
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
                vec![
                    doc! {
                        "$set": {
                            "position": {
                                "$replaceOne": {
                                    "input": "$position",
                                    "find": from,
                                    "replacement": to
                                }
                            }
                        }
                    },
                    doc! {
                        "$set": {
                            "fullpath": {
                                "$replaceOne": {
                                    "input": "$fullpath",
                                    "find": from,
                                    "replacement": to
                                }
                            }
                        }
                    },
                ],
                None,
            )
            .await?;
        Ok(())
    }
}

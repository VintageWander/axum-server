use crate::Result;

use super::FileService;

impl FileService {
    pub async fn change_inner_files_position(
        &self,
        search: &str,
        from: &str,
        to: &str,
    ) -> Result<()> {
        self.file_repo
            .change_inner_files_position(search, from, to)
            .await
    }
}

use crate::{services::Service, Result};

impl Service {
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

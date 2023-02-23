use std::io::Cursor;

use crate::{db::aws::S3, validation::file::check_full_filename, Result};
use axum::body::StreamBody;
use s3::Bucket;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

#[derive(Debug, Clone)]
pub struct Storage {
    storage: Bucket,
}

impl Storage {
    pub fn init(s3: &S3) -> Self {
        Self {
            storage: s3.get_storage(),
        }
    }

    // Use this function to get a list of bytes
    pub async fn get_object(&self, full_filename: &str) -> Result<Vec<u8>> {
        check_full_filename(full_filename)?;

        let bytes = self
            .storage
            .get_object(full_filename)
            .await?
            .bytes()
            .to_vec();

        Ok(bytes)
    }

    // Use this function in case you want to write directly to the response body
    // and return to the client
    pub async fn write_to_body(
        &self,
        full_filename: &str,
    ) -> Result<StreamBody<ReaderStream<Cursor<Vec<u8>>>>> {
        let mut writer = Cursor::new(Vec::new());

        self.storage
            .get_object_stream(full_filename, &mut writer)
            .await?;

        let stream = ReaderStream::new(writer);
        let body = StreamBody::new(stream);

        Ok(body)
    }

    // Use this function in case you want to get the object and write it to a file
    pub async fn write_to_file(&self, full_filename: &str, file: &mut File) -> Result<()> {
        self.storage
            .get_object_stream(full_filename, file)
            .await?;

        Ok(())
    }

    // Upload
    pub async fn put_object(&self, full_filename: &str, bytes: Vec<u8>) -> Result<()> {
        check_full_filename(full_filename)?;
        self.storage
            .put_object_stream(&mut Cursor::new(bytes), full_filename)
            .await?;

        Ok(())
    }

    pub async fn delete_object(&self, full_filename: &str) -> Result<()> {
        check_full_filename(full_filename)?;
        self.storage.delete_object(full_filename).await?;
        Ok(())
    }
}

use std::io::Cursor;

use crate::{db::aws::S3, validation::file::check_fullpath, Result};

use s3::{request::ResponseDataStream, Bucket};
use tokio::fs::File;

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
    pub async fn get_object(&self, full_filename: impl AsRef<str>) -> Result<Vec<u8>> {
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
    // pub async fn write_to_body(
    //     &self,
    //     full_filename: impl AsRef<str>,
    // ) -> Result<StreamBody<ReaderStream<Cursor<Vec<u8>>>>> {
    //     let mut writer = Cursor::new(Vec::new());

    //     self.storage
    //         .get_object_stream(full_filename, &mut writer)
    //         .await?;

    //     let stream = ReaderStream::new(writer);
    //     let body = StreamBody::new(stream);

    //     Ok(body)
    // }

    // Use this function in case you want to get the object and write it to a file
    pub async fn write_to_file(
        &self,
        full_filename: impl AsRef<str>,
        file: &mut File,
    ) -> Result<ResponseDataStream> {
        self.storage
            .get_object_stream(full_filename)
            .await
            .map_err(Into::into)
    }

    // Upload file
    pub async fn put_object(&self, full_filename: impl AsRef<str>, bytes: Vec<u8>) -> Result<()> {
        self.storage
            .put_object_stream(&mut Cursor::new(bytes), full_filename)
            .await?;

        Ok(())
    }

    // Create new folder
    pub async fn put_folder(&self, folder_name: impl AsRef<str>) -> Result<()> {
        self.storage
            .put_object_stream(
                &mut Cursor::new(Vec::new()),
                format!("{}/", folder_name.as_ref()),
            )
            .await?;

        Ok(())
    }

    // Move
    pub async fn move_object(&self, from: &str, to: &str) -> Result<()> {
        check_fullpath(from)?;
        check_fullpath(to)?;

        // It is basically a rename,
        // but S3 does not have a rename function

        // Our only solution is to copy it to a new place, and delete the old one

        // Put it to the new path
        self.storage
            .copy_object_internal(from, to)
            .await?;

        // Delete the old object
        self.delete_object(from).await?;

        Ok(())
    }

    // Delete
    pub async fn delete_object(&self, full_filename: impl AsRef<str>) -> Result<()> {
        self.storage.delete_object(full_filename).await?;
        Ok(())
    }

    pub async fn delete_folder(&self, folder_name: impl AsRef<str>) -> Result<()> {
        self.storage
            .delete_object(format!("{}/", folder_name.as_ref()))
            .await?;
        Ok(())
    }
}

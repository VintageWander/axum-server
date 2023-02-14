use std::vec::IntoIter;

use futures_util::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    options::{FindOneAndUpdateOptions, ReturnDocument},
    Collection,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{db::mongo::Mongo, error::Error, Result};

#[derive(Clone, Debug)]
pub struct Dao<T> {
    collection: Collection<T>,
}

impl<T> Dao<T>
where
    T: Serialize + DeserializeOwned + Unpin + Send + Sync + Into<Document>,
{
    pub fn init(db: &Mongo, collection_name: &str) -> Self {
        Self {
            collection: db.get_collection(collection_name),
        }
    }

    pub async fn get_multiple(&self, doc: Document) -> Result<IntoIter<T>> {
        let resources = self
            .collection
            .find(doc, None)
            .await?
            .try_collect::<Vec<_>>()
            .await?
            .into_iter();
        Ok(resources)
    }

    pub async fn get_one(&self, doc: Document) -> Result<T> {
        self.collection
            .find_one(doc, None)
            .await?
            .ok_or(Error::ResourceNotFound)
    }

    pub async fn exists_one(&self, doc: Document) -> Result<bool> {
        let count = self.collection.count_documents(doc, None).await?;
        Ok(count != 0)
    }

    pub async fn create_one(&self, resource: T) -> Result<T> {
        let new_resource_id = self
            .collection
            .insert_one(resource, None)
            .await?
            .inserted_id
            .as_object_id()
            .ok_or(Error::ResourceNotFound)?;
        self.get_one(doc! {"_id": new_resource_id}).await
    }

    // This update function calls the update_doc within the $set command,
    // You only need to provide it with the actual document that you want to change
    pub async fn update_one(&self, search_doc: Document, update_resource: T) -> Result<T> {
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        let resource_doc: Document = update_resource.into();

        self.collection
            .find_one_and_update(search_doc, doc! {"$set": resource_doc}, options)
            .await?
            .ok_or(Error::ResourceNotFound)
    }

    // This update function calls the update_doc within the $set command,
    // You only need to provide it with the actual document that you want to change
    pub async fn update_multiple(&self, search_doc: Document, update_doc: Document) -> Result<()> {
        self.collection
            .update_many(search_doc, doc! {"$set": update_doc}, None)
            .await?;
        Ok(())
    }

    pub async fn delete_one(&self, doc: Document) -> Result<()> {
        self.collection.delete_one(doc, None).await?;
        Ok(())
    }

    pub async fn delete_multiple(&self, doc: Document) -> Result<()> {
        self.collection.delete_many(doc, None).await?;
        Ok(())
    }
}

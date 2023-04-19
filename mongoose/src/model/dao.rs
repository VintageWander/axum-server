use quote::{format_ident, quote, ToTokens};
use syn::Ident;

pub fn dao_macro(struct_name: Ident) -> impl ToTokens {
    let dao_name = format_ident!("{}Dao", struct_name.to_string());
    let output = quote! {
        #[derive(Debug, Clone)]
        pub struct #dao_name {
            collection: mongodb::Collection<#struct_name>,
        }

        impl #dao_name {
            pub fn new(collection: mongodb::Collection<#struct_name>) -> Self {
                Self { collection }
            }

            pub async fn get_many(
                &self,
                filter: impl Into<mongodb::bson::Document>
            ) -> std::result::Result<Vec<#struct_name>, crate::error::Error> {
                use futures_util::stream::TryStreamExt;
                let results = self.collection.find(filter.into(), None).await?.try_collect().await?;
                Ok(results)
            }

            pub async fn get_one(
                &self,
                filter: impl Into<mongodb::bson::Document>
            ) -> std::result::Result<#struct_name, crate::error::Error> {
                self.collection.find_one(filter.into(), None).await?.ok_or(crate::error::Error::NotFound)
            }

            pub async fn exists_one(
                &self,
                filter: impl Into<mongodb::bson::Document>
            ) -> std::result::Result<bool, crate::error::Error> {
                let count = self.collection.count_documents(filter.into(), None).await?;
                Ok(count > 0)
            }

            pub async fn insert_one(
                &self,
                data: #struct_name
            ) -> std::result::Result<#struct_name, crate::error::Error> {
                let new_result_id =
                    self.collection
                        .insert_one(data, None)
                        .await?
                        .inserted_id
                        .as_object_id()
                        .ok_or(crate::error::Error::NotFound)?;
                self.collection
                    .find_one(
                        mongodb::bson::doc! { "_id": new_result_id },
                        None
                    )
                    .await?
                    .ok_or(crate::error::Error::NotFound)
            }

            pub async fn update_one(
                &self,
                data: #struct_name
            ) -> std::result::Result<#struct_name, crate::error::Error> {
                let filter = mongodb::bson::doc! { "_id": data.id };
                let options = mongodb::options::FindOneAndUpdateOptions::builder()
                                                                            .return_document(mongodb::options::ReturnDocument::After)
                                                                            .build();

                self.collection
                    .find_one_and_update(filter, mongodb::bson::doc! {"$set": data }, options)
                    .await?
                    .ok_or(crate::error::Error::NotFound)
            }

            pub async fn update_many(
                &self,
                search: impl Into<mongodb::bson::Document>,
                update: impl Into<mongodb::bson::Document>
            ) -> std::result::Result<(), crate::error::Error> {
                self.collection
                    .update_many(search.into(), mongodb::bson::doc!{"$set" : update.into()}, None)
                    .await?;
                Ok(())
            }

            pub async fn delete_one(
                &self,
                search: impl Into<mongodb::bson::Document>,
            ) -> std::result::Result<#struct_name, crate::error::Error> {
                let deleted_model = self
                                    .collection
                                    .find_one_and_delete(search.into(), None)
                                    .await?
                                    .ok_or(crate::error::Error::NotFound)?;
                Ok(deleted_model)
            }

            pub async fn delete_many(
                &self,
                search: impl Into<mongodb::bson::Document>
            ) -> std::result::Result<(), crate::error::Error> {
                self.collection.delete_many(search.into(), None).await?;
                Ok(())
            }
        }
    };
    output
}

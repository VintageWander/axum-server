use quote::{format_ident, quote, ToTokens};
use syn::{Fields, Ident};

pub fn into_search_query_macro(struct_type: Ident, fields: Fields) -> impl ToTokens {
    let query_from_request = format_ident!("{}QueryFromRequest", struct_type);

    let query_type = format_ident!("{}Query", struct_type);

    let mut query_from_request_fields = Vec::new();
    let mut doc_inserts = Vec::new();

    for field in fields {
        // Takes the field name
        let ident = field
            .ident
            .expect("Struct fields must have a name");

        // Takes the field type
        let ty = field.ty;

        // Convert the field name to string
        let ident_string = ident.to_string();

        // Convert the field type to string
        let ty_string = match &ty {
            syn::Type::Path(path) => path
                .path
                .segments
                .last()
                .unwrap()
                .ident
                .to_string(),
            _ => panic!("Only paths are supported"),
        };

        // Checks if the type is ObjectId
        let is_oid = ty_string == "ObjectId";

        // If the field is of type ObjectId
        //      Convert it into Option<String>
        // Else
        //      If the field is named "password", ignore the field
        //          Else Option<T>
        query_from_request_fields.push(match is_oid {
            true => match ident_string.as_str() {
                "id" => quote! {
                    #[serde(rename = "_id")]
                    pub id: Option<String>,
                },
                _ => quote! {
                    pub #ident: Option<String>,
                },
            },
            false => match ident_string.as_str() {
                "password" => quote! {},
                _ => quote! {
                    pub #ident: Option<#ty>,
                },
            },
        });

        // If the field is of type ObjectId
        //      If the field name is id
        //          insert value into "_id" key
        //      Else
        //          insert value into field name
        // Else
        //      If the field name is password
        //          Ignore
        //      Else
        //          insert value into field name as normal
        doc_inserts.push(match is_oid {
            true => match ident_string.as_str() {
                "id" => quote! {
                    if let Some(#ident) = query_from_request.#ident {
                        let oid = ObjectId::from_str(&#ident).map_err(|_| crate::Error::NotFound)?;
                        doc.insert("_id", oid);
                    }
                },
                _ => quote! {
                    if let Some(#ident) = query_from_request.#ident {
                        let oid = ObjectId::from_str(&#ident).map_err(|_| crate::Error::NotFound)?;
                        doc.insert(#ident_string, oid);
                    }
                },
            },
            false => match ident_string.as_str() {
                "password" => quote! {},
                _ => quote! {
                    if let Some(#ident) = query_from_request.#ident {
                        doc.insert(#ident_string, #ident);
                    }
                },
            },
        })
    }

    quote! {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct #query_from_request {
            #(#query_from_request_fields)*
        }

        #[axum::async_trait]
        impl axum::extract::FromRequest<crate::service::Service, axum::body::Body> for #query_from_request {
            type Rejection = crate::Error;
            async fn from_request(
                req: axum::http::Request<axum::body::Body>,
                state: &crate::service::Service,
            ) -> std::result::Result<Self, Self::Rejection> {
                                use mongodb::bson::Document;
                let axum::extract::Query(query) = axum::extract::Query::<#query_from_request>::from_request(req, state).await?;

                Ok(query)
            }
        }

        pub struct #query_type(pub ::mongodb::bson::Document);

        #[axum::async_trait]
        impl axum::extract::FromRequest<crate::service::Service, axum::body::Body> for #query_type {
            type Rejection = crate::Error;
            async fn from_request(
                req: axum::http::Request<axum::body::Body>,
                state: &crate::service::Service,
            ) -> std::result::Result<Self, Self::Rejection> {
                                use mongodb::bson::Document;
                let query_from_request = #query_from_request::from_request(req, state).await?;
                let mut doc: Document = Document::new();
                use std::str::FromStr;
                #(#doc_inserts)*
                Ok(Self(doc))
            }
        }
    }
}

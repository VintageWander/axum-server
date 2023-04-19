use quote::{quote, ToTokens};
use syn::{Fields, Ident};

use crate::snake_to_camel::snake_to_camel_case;

pub fn struct_into_doc(struct_type: Ident, fields: Fields) -> impl ToTokens {
    let mut doc_fields = Vec::new();

    for field in fields {
        let ident = field.ident.expect("Struct fields must have an identifier");

        let doc_field = snake_to_camel_case(ident.to_string());

        doc_fields.push(match doc_field == "id" {
            true => quote! {
                "_id": original.#ident,
            },
            false => quote! {
                #doc_field: original.#ident,
            },
        })
    }

    quote! {
        impl From<#struct_type> for mongodb::bson::Document {
            fn from(original: #struct_type) -> mongodb::bson::Document {
                use mongodb::bson::doc;
                doc! {
                    #(#doc_fields)*
                }
            }
        }

        impl From<#struct_type> for mongodb::bson::Bson {
            fn from(original: #struct_type) -> mongodb::bson::Bson {
                let doc: mongodb::bson::Document = original.into();
                doc.into()
            }
        }
    }
}

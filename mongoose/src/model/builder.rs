use quote::{format_ident, quote, ToTokens};
use syn::{Fields, Ident};

use crate::snake_to_camel::snake_to_camel_case;

pub fn builder_macro(struct_type: Ident, fields: &Fields) -> impl ToTokens {
    let builder_type = format_ident!("{}Builder", struct_type);

    let mut into_builder_methods = Vec::new();
    let mut builder_fields = Vec::new();
    let mut build_methods = Vec::new();
    let mut builder_to_struct_fields = Vec::new();
    let mut doc_inserts = Vec::new();
    let mut into_builder_fields = Vec::new();

    // Initialize into builder methods, converting from the original struct into builder struct
    for field in fields {
        let ident = field
            .ident
            .as_ref()
            .expect("Builder fields must have an identifier");
        let ty = &field.ty;

        let ident_string = ident.to_string();

        let ty_string = match &ty {
            syn::Type::Path(path) => path.path.segments.last().unwrap().ident.to_string(),
            _ => panic!("Only paths are supported"),
        };

        into_builder_methods.push(match ty_string.as_str() {
            "String" => quote! {
                pub fn #ident(#ident: &str) -> #builder_type {
                    <#builder_type>::default().#ident(#ident.into())
                }
            },
            _ => quote! {
                pub fn #ident(#ident: #ty) -> #builder_type {
                    <#builder_type>::default().#ident(#ident)
                }
            },
        });

        builder_fields.push(quote! {
            #ident: Option<#ty>
        });

        build_methods.push(match ty_string.as_str() {
            "String" => quote! {
                pub fn #ident(mut self, #ident: &str) -> Self {
                    self.#ident = Some(#ident.into());
                    self
                }
            },
            _ => quote! {
                pub fn #ident(mut self, #ident: #ty) -> Self {
                    self.#ident = Some(#ident);
                    self
                }
            },
        });

        builder_to_struct_fields.push(match ident_string.as_str() {
            "created_at" => quote! {
                created_at: builder.created_at.unwrap_or(chrono::Utc::now().timestamp_millis())
            },
            "updated_at" => quote! {
                updated_at: builder.updated_at.unwrap_or(chrono::Utc::now().timestamp_millis())
            },
            _ => quote! {
                #ident: builder.#ident.unwrap_or_default()
            },
        });

        let doc_field = snake_to_camel_case(format!("{}", ident));
        doc_inserts.push(match doc_field.as_str() {
            "id" => quote! {
                if let Some(#ident) = builder.#ident {
                    doc.insert("_id", #ident);
                }
            },
            _ => quote! {
                if let Some(#ident) = builder.#ident {
                    doc.insert(#doc_field, #ident);
                }
            },
        });

        into_builder_fields.push(quote! {
            #ident: Some(original.#ident)
        });
    }

    // The output
    let output = quote! {
        impl #struct_type {
            pub fn blank() -> #builder_type {
                #builder_type::default()
            }
            #(#into_builder_methods)*
            pub fn into_builder(self) -> #builder_type {
                self.into()
            }
        }

        #[derive(Default)]
        pub struct #builder_type {
            #(#builder_fields),*
        }

        impl #builder_type {
            #(#build_methods)*
            pub fn build(self) -> std::result::Result<#struct_type, crate::error::Error> {
                self.try_into()
            }
        }

        impl TryFrom<#builder_type> for #struct_type {
            type Error = crate::error::Error;

            fn try_from(builder: #builder_type) -> ::std::result::Result<Self, Self::Error> {
                let builded = #struct_type {
                    #(#builder_to_struct_fields),*
                };
                builded.validate()?;
                Ok(builded)
            }
        }

        impl From<#struct_type> for #builder_type {
            fn from(original: #struct_type) -> Self {
                Self {
                    #(#into_builder_fields),*
                }
            }
        }

        impl From<#builder_type> for mongodb::bson::Document {
            fn from(builder: #builder_type) -> mongodb::bson::Document {
                let mut doc = mongodb::bson::Document::new();
                #(#doc_inserts)*
                doc
            }
        }

        impl From<#struct_type> for mongodb::bson::Document {
            fn from(original: #struct_type) -> Self {
                let builder: #builder_type = original.into();
                builder.into()
            }
        }

        impl From<#struct_type> for mongodb::bson::Bson {
            fn from(original: #struct_type) -> Self {
                let doc: mongodb::bson::Document = original.into();
                doc.into()
            }
        }
    };

    output
}

use quote::{format_ident, quote, ToTokens};
use syn::{Fields, Ident};

pub fn dto_macro(struct_type: Ident, fields: Fields) -> impl ToTokens {
    let dto_type = format_ident!("{}DTO", struct_type);

    let mut dto_fields = Vec::new();
    let mut struct_to_dto_fields = Vec::new();

    for field in fields {
        let ident = field
            .ident
            .expect("#[derive(Dto)] only supports structs");
        let ident_string = ident.to_string();
        let ty = field.ty;

        let ty_string = match &ty {
            syn::Type::Path(path) => path
                .path
                .segments
                .last()
                .expect("Cannot get last segment")
                .ident
                .to_string(),
            _ => panic!("Only paths are supported"),
        };

        dto_fields.push(match ty_string.as_str() {
            "ObjectId" => match ident_string.as_str() {
                "id" => quote! {
                    #[serde(rename = "_id")]
                    pub #ident: String,
                },
                _ => quote! {
                    pub #ident: String,
                },
            },
            _ => match ident_string.as_str() {
                "password" => quote! {},
                "refresh_token" => quote! {},
                _ => quote! {
                    pub #ident: #ty,
                },
            },
        });

        struct_to_dto_fields.push(match ty_string.as_str() {
            "ObjectId" => quote! {
                #ident: original.#ident.to_string(),
            },
            _ => match ident_string.as_str() {
                "password" => quote! {},
                "refresh_token" => quote! {},
                _ => quote! {
                    #ident: original.#ident,
                },
            },
        })
    }
    quote! {
        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct #dto_type {
            #(#dto_fields)*
        }

        impl From<#struct_type> for #dto_type {
            fn from(original: #struct_type) -> Self {
                Self {
                    #(#struct_to_dto_fields)*
                }
            }
        }

        impl #struct_type {
            pub fn into_dto(self) -> #dto_type {
                self.into()
            }
        }
    }
}

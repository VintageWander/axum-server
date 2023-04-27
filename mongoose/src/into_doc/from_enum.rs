use quote::{quote, ToTokens};
use syn::Ident;

pub fn enum_into_doc(enum_type: Ident, variants: Vec<Ident>) -> impl ToTokens {
    let mut match_variants = Vec::new();
    let mut match_variants_to_str = Vec::new();
    for variant in variants {
        let variant_str = variant.to_string();

        let first_lowercase = variant_str
            .to_string()
            .chars()
            .next()
            .unwrap()
            .to_lowercase()
            .chain(variant_str.chars().skip(1))
            .collect::<String>();

        match_variants.push(quote! {
            #enum_type::#variant => #first_lowercase.into(),
        });

        match_variants_to_str.push(quote! {
            #enum_type::#variant => #first_lowercase,
        })
    }
    quote! {
        impl ToString for #enum_type {
            fn to_string(&self) -> std::string::String {
                match self {
                    #(#match_variants)*
                }
            }
        }

        impl From<#enum_type> for std::string::String {
            fn from(original: #enum_type) -> std::string::String {
                match original {
                    #(#match_variants)*
                }
            }
        }

        impl From<#enum_type> for &'static str {
            fn from(original: #enum_type) -> &'static str {
                match original {
                    #(#match_variants_to_str)*
                }
            }
        }

        impl From<#enum_type> for mongodb::bson::Bson {
            fn from(original: #enum_type) -> mongodb::bson::Bson {
                mongodb::bson::Bson::String(original.into())
            }
        }
    }
}

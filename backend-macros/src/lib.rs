mod dto;
mod into_search_query;

use dto::dto_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Dto)]
pub fn derive_dto(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_type = input.ident;

    match input.data {
        Data::Struct(data) => {
            let dto_macro = dto_macro(struct_type, data.fields);
            quote! { #dto_macro }.into()
        }
        _ => panic!("#[derive(Dto)] only supports structs"),
    }
}

#[proc_macro_derive(IntoSearchQuery)]
pub fn derive_into_search_query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_type = input.ident;

    match input.data {
        Data::Struct(data) => {
            let into_search_query_macro =
                into_search_query::into_search_query_macro(
                    struct_type,
                    data.fields,
                );
            quote! { #into_search_query_macro }.into()
        }
        _ => panic!("#[derive(IntoSearchQuery)] only supports structs"),
    }
}

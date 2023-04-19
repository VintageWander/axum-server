mod into_doc;
mod model;
mod snake_to_camel;

use into_doc::from_struct::struct_into_doc;
use model::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

use crate::into_doc::from_enum::enum_into_doc;

#[proc_macro_derive(Model)]
pub fn model_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_type = input.ident;

    let fields = match input.data {
        Data::Struct(data) => data.fields,
        _ => panic!("#[derive(Model)] only supports structs"),
    };

    let model_macro = model_macro(struct_type, &fields);

    quote! {
        #model_macro
    }
    .into()
}

#[proc_macro_derive(IntoDoc)]
pub fn into_doc_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let derive_type = input.ident;

    match input.data {
        Data::Struct(data) => {
            let struct_macro = struct_into_doc(derive_type, data.fields);
            quote! { #struct_macro }
        }
        Data::Enum(data) => {
            let enum_macro = enum_into_doc(
                derive_type,
                data.variants
                    .iter()
                    .map(|v| v.ident.clone())
                    .collect::<Vec<_>>(),
            );
            quote! { #enum_macro }
        }
        _ => panic!("#[derive(IntoDoc)] only supports structs and enums"),
    }
    .into()
}

/*
    To get the path of the enum, you can use the `path` method of the `DeriveInput` struct. So you can replace `${INSERT_HERE}` with the following code:

```rust
let enum_path = input.path;



*/

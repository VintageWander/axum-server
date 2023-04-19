use quote::{quote, ToTokens};
use syn::{Fields, Ident};

use crate::model::{builder::builder_macro, dao::dao_macro};

pub mod builder;
pub mod dao;

pub fn model_macro(struct_type: Ident, fields: &Fields) -> impl ToTokens {
    let builder_macro = builder_macro(struct_type.clone(), fields);
    let dao_macro = dao_macro(struct_type);
    quote! {
        #dao_macro
        #builder_macro
    }
}

mod field;
mod fields;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Fields, ItemStruct};

pub fn derive(input: ItemStruct) -> Result<TokenStream, syn::Error> {
    let name = &input.ident;

    Ok(quote! {})
}

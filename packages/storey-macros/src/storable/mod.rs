mod field;
mod fields;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{token::Token, Fields, ItemStruct};

pub fn derive(input: ItemStruct) -> Result<TokenStream, syn::Error> {
    let name = &input.ident;

    let fields = fields::Fields::new(&input.fields)?.fields;

    let constructor = derive_constructor()?;
    let accessor = derive_accessor()?;
    let storable_impl = derive_storable_impl()?;

    Ok(quote! {
        #constructor
        #accessor
        #storable_impl
    })
}

fn derive_constructor() -> Result<TokenStream, syn::Error> {
    todo!()
}

fn derive_accessor() -> Result<TokenStream, syn::Error> {
    todo!()
}

fn derive_storable_impl() -> Result<TokenStream, syn::Error> {
    todo!()
}

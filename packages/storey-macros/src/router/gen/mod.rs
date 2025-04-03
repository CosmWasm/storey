use proc_macro2::TokenStream;
use quote::quote;

use super::def::RouterDef;

pub fn gen(_def: RouterDef) -> Result<TokenStream, syn::Error> {
    let struct_def = derive_struct()?;
    let constructor = derive_constructor()?;
    let storable_impl = derive_storable_impl()?;
    let accessor_def = derive_accessor()?;
    let access_methods = derive_access_methods()?;

    Ok(quote! {
        #struct_def
        #constructor
        #storable_impl
        #accessor_def
        #access_methods
    })
}

fn derive_struct() -> Result<TokenStream, syn::Error> {
    todo!()
}

fn derive_constructor() -> Result<TokenStream, syn::Error> {
    todo!()
}

fn derive_storable_impl() -> Result<TokenStream, syn::Error> {
    todo!()
}

fn derive_accessor() -> Result<TokenStream, syn::Error> {
    todo!()
}

fn derive_access_methods() -> Result<TokenStream, syn::Error> {
    todo!()
}

mod key_derive;
mod router;

use proc_macro::TokenStream;

#[proc_macro]
pub fn router(input: TokenStream) -> TokenStream {
    let expanded = match router::entry(input.into()) {
        Ok(res) => res,
        Err(e) => e.into_compile_error(),
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Key)]
pub fn key_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemStruct);

    let expanded = match key_derive::key_derive(input) {
        Ok(res) => res,
        Err(e) => e.into_compile_error(),
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(OwnedKey)]
pub fn owned_key_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemStruct);

    let expanded = match key_derive::owned_key_derive(input) {
        Ok(res) => res,
        Err(e) => e.into_compile_error(),
    };

    TokenStream::from(expanded)
}

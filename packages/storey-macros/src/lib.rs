mod key_derive;

#[proc_macro_derive(Key)]
pub fn key_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemStruct);

    let expanded = match key_derive::derive(input) {
        Ok(res) => res,
        Err(e) => e.into_compile_error(),
    };

    proc_macro::TokenStream::from(expanded)
}

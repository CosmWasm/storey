use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, spanned::Spanned, Fields, Ident, ItemStruct};

pub fn derive(input: ItemStruct) -> Result<TokenStream, syn::Error> {
    let name = &input.ident;

    let inner_type = extract_newtype(&input)?;

    Ok(quote! {
        impl<KS> ::storey::containers::map::Key<KS> for #name {
            type Kind = <#inner_type as Key<KS>>::Kind;

            fn encode(&self) -> Vec<u8> {
                ::storey::containers::map::Key::<KS>::encode(&self.0)
            }
        }
    })
}

fn extract_newtype(input: &ItemStruct) -> Result<syn::Type, syn::Error> {
    if let Fields::Unnamed(fields) = &input.fields {
        let fields: Vec<_> = fields.unnamed.iter().collect();

        if let [field] = fields.as_slice() {
            return Ok(field.ty.clone());
        }
    }

    Err(syn::Error::new_spanned(
        &input.ident,
        "the Key derive only accepts newtype structs",
    ))
}

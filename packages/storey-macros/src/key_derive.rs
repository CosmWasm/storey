use proc_macro2::TokenStream;
use quote::quote;
use syn::{Fields, ItemStruct};

pub fn key_derive(input: ItemStruct) -> Result<TokenStream, syn::Error> {
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

pub fn owned_key_derive(input: ItemStruct) -> Result<TokenStream, syn::Error> {
    let name = &input.ident;

    let inner_type = extract_newtype(&input)?;

    Ok(quote! {
        impl<KS> ::storey::containers::map::OwnedKey<KS> for #name {
            type Error = <#inner_type as OwnedKey<KS>>::Error;

            fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
                ::storey::containers::map::OwnedKey::<KS>::from_bytes(bytes).map(Self)
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

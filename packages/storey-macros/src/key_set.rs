use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Ident, ItemStruct};

pub fn derive(input: ItemStruct) -> Result<TokenStream, syn::Error> {
    let name = &input.ident;

    let key_impls = derive_key_impls(name);
    let owned_key_impls = derive_owned_key_impls(name);
    let arrays = derive_arrays(name);

    Ok(quote! {
        #key_impls
        #owned_key_impls
        #arrays
    })
}

pub fn derive_key_impls(name: &Ident) -> TokenStream {
    let mut types = get_owned_delegations();
    types.extend(get_key_only_delegations());

    quote! {
        #(
            impl Key<#name> for #types {
                type Kind = <#types as Key>::Kind;

                fn encode(&self) -> Vec<u8> {
                    <#types as Key>::encode(self)
                }
            }
        )*
    }
}

pub fn derive_owned_key_impls(name: &Ident) -> TokenStream {
    let types = get_owned_delegations();

    quote! {
        #(
            impl OwnedKey<#name> for #types {
                type Error = <#types as OwnedKey>::Error;

                fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
                where
                    Self: Sized,
                {
                    <#types as OwnedKey>::from_bytes(bytes)
                }
            }
        )*
    }
}

pub fn derive_arrays(name: &Ident) -> TokenStream {
    quote::quote! {
        impl<const N: usize> Key<#name> for [u8; N] {
            type Kind = <Self as Key>::Kind;

            fn encode(&self) -> Vec<u8> {
                <Self as Key>::encode(self)
            }
        }

        impl<const N: usize> OwnedKey<#name> for [u8; N] {
            type Error = <Self as OwnedKey>::Error;

            fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
            where
                Self: Sized,
            {
                <Self as OwnedKey>::from_bytes(bytes)
            }
        }
    }
}

fn get_owned_delegations() -> Vec<syn::Type> {
    // these implement both Key and OwnedKey

    vec![
        parse_quote!(String),
        parse_quote!(Box<str>),
        parse_quote!(Vec<u8>),
        parse_quote!(Box<[u8]>),
        parse_quote!(u8),
        parse_quote!(u16),
        parse_quote!(u32),
        parse_quote!(u64),
        parse_quote!(u128),
        parse_quote!(i8),
        parse_quote!(i16),
        parse_quote!(i32),
        parse_quote!(i64),
        parse_quote!(i128),
    ]
}

fn get_key_only_delegations() -> Vec<syn::Type> {
    // these implement Key but not OwnedKey

    vec![parse_quote!(str), parse_quote!([u8])]
}

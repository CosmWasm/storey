use proc_macro2::TokenStream;

use super::def::{Field, RouterDef};

pub fn parse(_input: TokenStream) -> Result<RouterDef, syn::Error> {
    // mock implementation for testing!
    Ok(RouterDef {
        name: syn::Ident::new("Foo", proc_macro2::Span::call_site()),
        accessor_name: syn::Ident::new("FooAccess", proc_macro2::Span::call_site()),
        fields: vec![
            Field {
                ty: syn::parse_str("Item<u64, TestEncoding>").unwrap(),
                name: syn::Ident::new("a", proc_macro2::Span::call_site()),
                key: 0,
            },
            Field {
                ty: syn::parse_str("Map<String, Item<u64, TestEncoding>>").unwrap(),
                name: syn::Ident::new("b", proc_macro2::Span::call_site()),
                key: 1,
            },
            Field {
                ty: syn::parse_str("Item<u64, TestEncoding>").unwrap(),
                name: syn::Ident::new("c", proc_macro2::Span::call_site()),
                key: 2,
            },
        ],
    })
}

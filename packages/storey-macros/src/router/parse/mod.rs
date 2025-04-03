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

#[cfg(test)]
mod tests {
    use super::*;

    use quote::{quote, ToTokens as _};

    #[test]
    fn basic_parse() {
        let input = quote! {
            router Foo {
                0 -> a: Item<u64, TestEncoding>,
                1 -> b: Map<String, Item<u64, TestEncoding>>,
                2 -> c: Item<u64, TestEncoding>,
            }
        };

        let def = parse(input).unwrap();

        assert_eq!(def.name.to_string(), "Foo");
        assert_eq!(def.accessor_name.to_string(), "FooAccess");
        assert_eq!(def.fields.len(), 3);
        assert_eq!(def.fields[0].name.to_string(), "a");
        assert_eq!(
            def.fields[0].ty.to_token_stream().to_string(),
            "Item < u64 , TestEncoding >"
        );
        assert_eq!(def.fields[0].key, 0);
        assert_eq!(def.fields[1].name.to_string(), "b");
        assert_eq!(
            def.fields[1].ty.to_token_stream().to_string(),
            "Map < String , Item < u64 , TestEncoding > >"
        );
        assert_eq!(def.fields[1].key, 1);
        assert_eq!(def.fields[2].name.to_string(), "c");
        assert_eq!(
            def.fields[2].ty.to_token_stream().to_string(),
            "Item < u64 , TestEncoding >"
        );
        assert_eq!(def.fields[2].key, 2);
    }
}

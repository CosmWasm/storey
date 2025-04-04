mod field;
mod fields;

use fields::Fields;
use proc_macro2::TokenStream;
use quote::format_ident;
use syn::{braced, parse::Parse};

use super::def::{Field, RouterDef};

pub fn parse(input: TokenStream) -> Result<RouterDef, syn::Error> {
    let input = syn::parse2::<Def>(input)?;
    let accessor_name = format_ident!("{}Access", input.name);

    let mut fields = Vec::new();
    for field in input.fields.fields {
        fields.push(Field {
            key: field.key,
            name: field.name,
            ty: field.ty,
        });
    }

    Ok(RouterDef {
        name: input.name,
        accessor_name,
        fields,
    })
}

struct Def {
    name: syn::Ident,
    fields: Fields,
}

impl Parse for Def {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let router_keyword = input.parse::<syn::Ident>()?;
        if router_keyword != "router" {
            return Err(syn::Error::new(
                router_keyword.span(),
                "Expected `router` keyword",
            ));
        }

        let name: syn::Ident = input.parse()?;

        let fields;
        braced!(fields in input);
        let fields: Fields = fields.parse()?;

        Ok(Def { name, fields })
    }
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

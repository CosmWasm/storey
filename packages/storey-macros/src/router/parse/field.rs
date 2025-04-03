use syn::parse::Parse;

pub struct Field {
    pub ty: syn::Type,
    pub name: syn::Ident,
    pub key: u8,
}

impl Parse for Field {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key: syn::LitInt = input.parse()?;
        input.parse::<syn::Token![->]>()?;
        let name: syn::Ident = input.parse()?;
        input.parse::<syn::Token![:]>()?;
        let ty: syn::Type = input.parse()?;

        Ok(Field {
            ty,
            name,
            key: key.base10_parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens as _;

    use super::*;

    #[test]
    fn test_field_parse() {
        let field: Field = syn::parse_str("0 -> a: Item<u64, TestEncoding>").unwrap();

        assert_eq!(field.key, 0);
        assert_eq!(field.name.to_string(), "a");
        assert_eq!(
            field.ty.to_token_stream().to_string(),
            "Item < u64 , TestEncoding >"
        );
    }
}

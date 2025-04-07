use std::collections::{HashSet, VecDeque};

use syn::parse::Parse;

use super::field::Field;

pub struct Fields {
    pub fields: Vec<Field>,
}

impl Parse for Fields {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut fields = Vec::new();

        while !input.is_empty() {
            let field: Field = input.parse()?;

            fields.push(field);
            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }

        validate_fields(fields.as_slice())?;

        Ok(Fields { fields })
    }
}

fn validate_fields(fields: &[Field]) -> syn::Result<()> {
    let mut names = HashSet::new();
    let mut keys = HashSet::new();
    let mut errors = VecDeque::new();

    for field in fields {
        if names.contains(&field.name) {
            errors.push_back(syn::Error::new(
                field.name.span(),
                format!("Duplicate field name: {}", field.name),
            ));
        }

        if keys.contains(&field.key) {
            errors.push_back(syn::Error::new(
                field.key_span,
                format!("Duplicate field key: {}", field.key),
            ));
        }

        if field.key == 255 {
            errors.push_back(syn::Error::new(
                field.key_span,
                "Key 255 is reserved for metadata",
            ));
        }

        names.insert(field.name.clone());
        keys.insert(field.key);
    }

    if !errors.is_empty() {
        let mut combined_error = errors.pop_front().unwrap();
        combined_error.extend(errors);

        Err(combined_error)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens as _;

    use super::*;

    #[test]
    fn test_fields_parse() {
        let input = syn::parse_str("0 -> a: Item<u64, TestEncoding>, 1 -> b: Map<String, Item<u64, TestEncoding>>, 2 -> c: Item<u64, TestEncoding>").unwrap();
        let fields: Fields = syn::parse2(input).unwrap();

        assert_eq!(fields.fields.len(), 3);

        assert_eq!(fields.fields[0].key, 0);
        assert_eq!(fields.fields[0].name.to_string(), "a");
        assert_eq!(
            fields.fields[0].ty.to_token_stream().to_string(),
            "Item < u64 , TestEncoding >"
        );

        assert_eq!(fields.fields[1].key, 1);
        assert_eq!(fields.fields[1].name.to_string(), "b");
        assert_eq!(
            fields.fields[1].ty.to_token_stream().to_string(),
            "Map < String , Item < u64 , TestEncoding > >"
        );

        assert_eq!(fields.fields[2].key, 2);
        assert_eq!(fields.fields[2].name.to_string(), "c");
        assert_eq!(
            fields.fields[2].ty.to_token_stream().to_string(),
            "Item < u64 , TestEncoding >"
        );
    }
}

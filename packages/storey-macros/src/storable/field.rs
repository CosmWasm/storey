use syn::{LitInt, Meta};

pub(crate) struct Field {
    pub ty: syn::Type,
    pub key: u8,
}

impl Field {
    pub fn new(field: &syn::Field) -> syn::Result<Self> {
        let ty = field.ty.clone();

        let key = parse_key(field)?;

        Ok(Self { ty, key })
    }
}

fn parse_key(field: &syn::Field) -> syn::Result<u8> {
    let key_attrs: Vec<_> = field
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("key"))
        .collect();

    let attr = match &key_attrs[..] {
        [attr] => attr,
        [] => return Err(syn::Error::new_spanned(field, "missing key attribute")),
        _ => {
            return Err(syn::Error::new_spanned(
                field,
                "multiple key attributes found",
            ))
        }
    };

    parse_meta(&attr.meta)
}

fn parse_meta(meta: &syn::Meta) -> syn::Result<u8> {
    if let Meta::List(meta_list) = meta {
        if let Ok(key_lit) = syn::parse2::<LitInt>(meta_list.tokens.clone()) {
            if let Ok(key) = key_lit.base10_parse::<u8>() {
                return Ok(key);
            }
        }
    }

    Err(syn::Error::new_spanned(
        meta,
        "expected a number between 0-255",
    ))
}

#[cfg(test)]
mod tests {
    use quote::ToTokens as _;

    use super::*;

    #[test]
    fn parse_named_field() {
        let field: syn::FieldsNamed = syn::parse_str(
            r#"
            {
            #[key(1)]
            pub name: String,
            }
        "#,
        )
        .unwrap();

        let field = field.named.first().unwrap();
        let field = Field::new(field).unwrap();

        assert_eq!(field.key, 1);
        assert_eq!(field.ty.to_token_stream().to_string(), "String");
    }

    #[test]
    fn parse_fails_with_no_key_attr() {
        let field: syn::FieldsNamed = syn::parse_str(
            r#"
            {
            pub name: String,
            }
        "#,
        )
        .unwrap();

        let field = field.named.first().unwrap();

        if let Err(err) = Field::new(field) {
            assert_eq!(err.to_string(), "missing key attribute");
        } else {
            panic!("expected error");
        }
    }

    #[test]
    fn parse_fails_with_path_like_key_attr() {
        let field: syn::FieldsNamed = syn::parse_str(
            r#"
            {
            #[key]
            pub name: String,
            }
        "#,
        )
        .unwrap();

        let field = field.named.first().unwrap();

        if let Err(err) = Field::new(field) {
            assert_eq!(err.to_string(), "expected a number between 0-255");
        } else {
            panic!("expected error");
        }
    }

    #[test]
    fn parse_fails_with_arbitrary_tokens() {
        let field: syn::FieldsNamed = syn::parse_str(
            r#"
            {
            #[key("foo")]
            pub name: String,
            #[key(bar, baz)]
            pub id: u32,
            }
        "#,
        )
        .unwrap();

        let fields = field.named.iter().collect::<Vec<_>>();

        if let Err(err) = Field::new(fields[0]) {
            assert_eq!(err.to_string(), "expected a number between 0-255");
        } else {
            panic!("expected error");
        }

        if let Err(err) = Field::new(fields[1]) {
            assert_eq!(err.to_string(), "expected a number between 0-255");
        } else {
            panic!("expected error");
        }
    }

    #[test]
    fn parse_fails_with_bad_number() {
        let field: syn::FieldsNamed = syn::parse_str(
            r#"
            {
            #[key(-1)]
            pub name: String,
            #[key(256)]
            pub id: u32,
            }
        "#,
        )
        .unwrap();

        let fields = field.named.iter().collect::<Vec<_>>();

        if let Err(err) = Field::new(fields[0]) {
            assert_eq!(err.to_string(), "expected a number between 0-255");
        } else {
            panic!("expected error");
        }

        if let Err(err) = Field::new(fields[1]) {
            assert_eq!(err.to_string(), "expected a number between 0-255");
        } else {
            panic!("expected error");
        }
    }
}

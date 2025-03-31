use super::field::Field;

pub(crate) struct Fields {
    pub fields: Vec<Field>,
}

impl Fields {
    pub fn new(fields: &Fields) -> Result<Self, syn::Error> {
        todo!()
    }
}

impl syn::parse::Parse for Fields {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

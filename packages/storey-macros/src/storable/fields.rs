use super::field::Field;

pub(crate) struct Fields {
    pub fields: Vec<Field>,
}

impl Fields {
    pub fn new(fields: &syn::Fields) -> Result<Self, syn::Error> {
        todo!()
    }
}

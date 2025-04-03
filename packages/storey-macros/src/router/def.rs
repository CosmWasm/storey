pub struct RouterDef {
    pub name: syn::Ident,
    pub accessor_name: syn::Ident,
    pub fields: Vec<Field>,
}

pub struct Field {
    pub ty: syn::Type,
    pub name: syn::Ident,
    pub key: u8,
}

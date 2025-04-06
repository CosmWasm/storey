use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::def::RouterDef;

pub fn gen(def: RouterDef) -> Result<TokenStream, syn::Error> {
    let struct_def = derive_struct(&def);
    let constructor = derive_constructor(&def);
    let storable_impl = derive_storable_impl(&def);
    let accessor_def = derive_accessor(&def);
    let access_methods = derive_access_methods(&def);

    Ok(quote! {
        #struct_def
        #constructor
        #storable_impl
        #accessor_def
        #access_methods
    })
}

fn derive_struct(def: &RouterDef) -> TokenStream {
    let name = &def.name;

    quote! {
        pub struct #name;
    }
}

fn derive_constructor(def: &RouterDef) -> TokenStream {
    let name = &def.name;
    let accessor_name = &def.accessor_name;

    quote! {
        impl #name {
            pub fn access<F, S>(storage: F) -> #accessor_name<::storey::storage::StorageBranch<S>>
            where
                (F,): ::storey::storage::IntoStorage<S>,
            {
                let storage = ::storey::storage::IntoStorage::into_storage((storage,));
                <Self as ::storey::containers::Storable>::access_impl(::storey::storage::StorageBranch::new(storage, vec![]))
            }
        }
    }
}

fn derive_storable_impl(def: &RouterDef) -> TokenStream {
    let name = &def.name;
    let accessor_name = &def.accessor_name;

    quote! {
        impl ::storey::containers::Storable for #name {
            type Kind = ::storey::containers::NonTerminal;
            type Accessor<S> = #accessor_name<S>;

            fn access_impl<S>(storage: S) -> Self::Accessor<S> {
                Self::Accessor { storage }
            }
        }
    }
}

fn derive_accessor(def: &RouterDef) -> TokenStream {
    let accessor_name = &def.accessor_name;

    quote! {
        pub struct #accessor_name<S> {
            storage: S,
        }
    }
}

fn derive_access_methods(def: &RouterDef) -> TokenStream {
    let accessor_name = &def.accessor_name;

    let f_names = def.fields.iter().map(|f| f.name.clone());
    let f_names_mut = def.fields.iter().map(|f| format_ident!("{}_mut", f.name));
    let keys = def.fields.iter().map(|f| f.key);
    let types = def.fields.iter().map(|f| f.ty.clone());

    quote! {
        impl<S> #accessor_name<S> {
            #(
                pub fn #f_names(&self) -> <#types as ::storey::containers::Storable>::Accessor<::storey::storage::StorageBranch<&S>> {
                    <#types as ::storey::containers::Storable>::access_impl(::storey::storage::StorageBranch::new(&self.storage, vec![#keys]))
                }

                pub fn #f_names_mut(
                    &mut self,
                ) -> <#types as ::storey::containers::Storable>::Accessor<::storey::storage::StorageBranch<&mut S>> {
                    <#types as ::storey::containers::Storable>::access_impl(::storey::storage::StorageBranch::new(&mut self.storage, vec![#keys]))
                }
            )*
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::router::def::Field;

    use super::*;

    #[test]
    fn struc() {
        let def = RouterDef {
            name: syn::Ident::new("Foo", proc_macro2::Span::call_site()),
            accessor_name: syn::Ident::new("FooAccess", proc_macro2::Span::call_site()),
            fields: vec![],
        };

        let generated = derive_struct(&def);
        let expected = quote! {
            pub struct Foo;
        };

        assert_eq!(generated.to_string(), expected.to_string());
    }

    #[test]
    fn constructor() {
        let def = RouterDef {
            name: syn::Ident::new("Foo", proc_macro2::Span::call_site()),
            accessor_name: syn::Ident::new("FooAccess", proc_macro2::Span::call_site()),
            fields: vec![],
        };

        let generated = derive_constructor(&def);
        let expected = quote! {
            impl Foo {
                pub fn access<F, S>(storage: F) -> FooAccess<::storey::storage::StorageBranch<S>>
                where
                    (F,): ::storey::storage::IntoStorage<S>,
                {
                    let storage = ::storey::storage::IntoStorage::into_storage((storage,));
                    <Self as ::storey::containers::Storable>::access_impl(::storey::storage::StorageBranch::new(storage, vec![]))
                }
            }
        };

        assert_eq!(generated.to_string(), expected.to_string());
    }

    #[test]
    fn storable_impl() {
        let def = RouterDef {
            name: syn::Ident::new("Foo", proc_macro2::Span::call_site()),
            accessor_name: syn::Ident::new("FooAccess", proc_macro2::Span::call_site()),
            fields: vec![],
        };

        let generated = derive_storable_impl(&def);
        let expected = quote! {
            impl ::storey::containers::Storable for Foo {
                type Kind = ::storey::containers::NonTerminal;
                type Accessor<S> = FooAccess<S>;

                fn access_impl<S>(storage: S) -> Self::Accessor<S> {
                    Self::Accessor { storage }
                }
            }
        };

        assert_eq!(generated.to_string(), expected.to_string());
    }

    #[test]
    fn accessor_def() {
        let def = RouterDef {
            name: syn::Ident::new("Foo", proc_macro2::Span::call_site()),
            accessor_name: syn::Ident::new("FooAccess", proc_macro2::Span::call_site()),
            fields: vec![],
        };

        let generated = derive_accessor(&def);
        let expected = quote! {
            pub struct FooAccess<S> {
                storage: S,
            }
        };

        assert_eq!(generated.to_string(), expected.to_string());
    }

    #[test]
    fn access_methods() {
        let def = RouterDef {
            name: syn::Ident::new("Foo", proc_macro2::Span::call_site()),
            accessor_name: syn::Ident::new("FooAccess", proc_macro2::Span::call_site()),
            fields: vec![Field {
                ty: syn::parse_str("Item<u64, TestEncoding>").unwrap(),
                name: syn::Ident::new("a", proc_macro2::Span::call_site()),
                key: 2,
            }],
        };

        let generated = derive_access_methods(&def);
        let expected = quote! {
            impl<S> FooAccess<S> {
                pub fn a(&self) -> <Item<u64, TestEncoding> as ::storey::containers::Storable>::Accessor<::storey::storage::StorageBranch<&S>> {
                    <Item<u64, TestEncoding> as ::storey::containers::Storable>::access_impl(::storey::storage::StorageBranch::new(&self.storage, vec![2u8]))
                }

                pub fn a_mut(
                    &mut self,
                ) -> <Item<u64, TestEncoding> as ::storey::containers::Storable>::Accessor<::storey::storage::StorageBranch<&mut S>> {
                    <Item<u64, TestEncoding> as ::storey::containers::Storable>::access_impl(::storey::storage::StorageBranch::new(&mut self.storage, vec![2u8]))
                }
            }
        };
        assert_eq!(generated.to_string(), expected.to_string());
    }
}

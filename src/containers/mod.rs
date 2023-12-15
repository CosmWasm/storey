use crate::{
    backend::StorageBackend,
    encoding::{DecodableWith, EncodableWith, Encoding},
};

mod item;

pub trait Container<E: Encoding> {
    type AccessorT<'ns>: Accessor<E, Item = Self::Item>
    where
        E: 'ns,
        Self::Item: 'ns;
    type Item: EncodableWith<E> + DecodableWith<E>;

    fn init(ns: &[u8], _storage: &mut impl StorageBackend) -> Result<(), E::EncodeError> {
        Ok(())
    }

    fn access(prefix: &[u8]) -> Self::AccessorT<'_>;
}

pub trait Accessor<E: Encoding> {
    type Item;
}

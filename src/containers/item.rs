use std::borrow::{Borrow, Cow};
use std::marker::PhantomData;

use crate::{
    encoding::{DecodableWith, EncodableWith, Encoding},
    StorageBackend, StorageBackendMut,
};

use super::{Accessor, Storable};

pub struct Item<T, E> {
    prefix: &'static [u8],
    phantom: PhantomData<(T, E)>,
}

impl<T, E> Item<T, E>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
{
    pub const fn new(prefix: &'static [u8]) -> Self {
        Self {
            prefix,
            phantom: PhantomData,
        }
    }

    pub fn access(&self) -> ItemAccess<'static, E, T> {
        Self::access_impl(self.prefix)
    }
}

impl<T, E> Storable<E> for Item<T, E>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
{
    type AccessorT<'ns> = ItemAccess<'ns, E, T>;

    fn access_impl<'k>(prefix: impl Into<Cow<'k, [u8]>>) -> ItemAccess<'k, E, T> {
        ItemAccess {
            prefix: prefix.into(),
            phantom: PhantomData,
        }
    }
}

pub struct ItemAccess<'k, E, T> {
    prefix: Cow<'k, [u8]>,
    phantom: PhantomData<(E, T)>,
}

impl<E, T> ItemAccess<'_, E, T>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
{
    pub fn get(&self, storage: &impl StorageBackend) -> Result<Option<T>, E::DecodeError> {
        storage
            .get(self.prefix.borrow())
            .map(|bytes| T::decode(bytes.as_slice()))
            .transpose()
    }

    pub fn set(&self, storage: &impl StorageBackendMut, value: &T) -> Result<(), E::EncodeError> {
        let bytes = value.encode()?;
        storage.set(self.prefix.borrow(), &bytes);
        Ok(())
    }
}

impl<E, T> Accessor for ItemAccess<'_, E, T> {}

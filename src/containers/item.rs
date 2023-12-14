use std::marker::PhantomData;

use crate::backend::StorageBackend;
use crate::encoding::Encoding;
use crate::{DecodableWith, EncodableWith};

use super::Container;

struct Item<'k, E, T> {
    prefix: &'k [u8],
    phantom: PhantomData<(E, T)>,
}

impl<'k, E, T> Item<'k, E, T>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E>,
{
    pub fn new(prefix: &'k [u8]) -> Self {
        Self {
            prefix,
            phantom: PhantomData,
        }
    }

    pub fn get(&self, storage: &mut impl StorageBackend, key: &[u8]) -> Result<T, E::DecodeError> {
        let data = storage.get(key).unwrap();
        let item = T::decode(&data)?;
        Ok(item)
    }

    pub fn set(&self, storage: &mut impl StorageBackend, item: &T) -> Result<(), E::EncodeError> {
        let data = item.encode()?;
        storage.set(self.prefix, &data);

        Ok(())
    }
}

impl<E, T> Container<E> for Item<'_, E, T>
where
    E: Encoding,
    T: EncodableWith<E> + DecodableWith<E> + Default,
{
    type Item = T;

    fn init(&self, storage: &mut impl StorageBackend) -> Result<(), E::EncodeError> {
        self.set(storage, &T::default())
    }
}

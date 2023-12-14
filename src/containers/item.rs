use std::marker::PhantomData;

use crate::backend::StorageBackend;
use crate::encoding::Encoding;

use super::Container;

struct Item<'k, E> {
    prefix: &'k [u8],
    phantom: PhantomData<E>,
}

impl<'k, E> Item<'k, E>
where
    E: Encoding,
{
    pub fn new(prefix: &'k [u8]) -> Self {
        Self {
            prefix,
            phantom: PhantomData,
        }
    }

    pub fn get(
        &self,
        storage: &mut impl StorageBackend,
        key: &[u8],
    ) -> Result<E::Type, E::DecodeError> {
        let data = storage.get(key).unwrap();
        let item = E::decode(&data)?;
        Ok(item)
    }

    pub fn set(
        &self,
        storage: &mut impl StorageBackend,
        item: &E::Type,
    ) -> Result<(), E::EncodeError> {
        let data = E::encode(item)?;
        storage.set(self.prefix, &data);

        Ok(())
    }
}

impl<E> Container<E> for Item<'_, E>
where
    E: Encoding,
    E::Type: Default,
{
    fn init(&self, storage: &mut impl StorageBackend) -> Result<(), E::EncodeError> {
        self.set(storage, &E::Type::default())
    }
}

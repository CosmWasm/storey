use std::ops::Bound;

use storey::storage::{
    IntoStorage, IterableStorage, RevIterableStorage, StorageBackend, StorageBackendMut,
};

/// A wrapper around a type implementing [`cosmwasm_std::Storage`] that integrates it with [`storey`].
#[repr(transparent)]
pub struct CwStorage<S: ?Sized>(pub S);

impl<S> StorageBackend for CwStorage<S>
where
    S: cosmwasm_std::Storage + ?Sized,
{
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        cosmwasm_std::Storage::get(&self.0, key)
    }
}

impl<S> StorageBackendMut for CwStorage<S>
where
    S: cosmwasm_std::Storage + ?Sized,
{
    fn set(&mut self, key: &[u8], value: &[u8]) {
        cosmwasm_std::Storage::set(&mut self.0, key, value)
    }

    fn remove(&mut self, key: &[u8]) {
        cosmwasm_std::Storage::remove(&mut self.0, key)
    }
}

impl<S> IterableStorage for CwStorage<S>
where
    S: cosmwasm_std::Storage + ?Sized,
{
    type KeysIterator<'a>
        = Box<dyn Iterator<Item = Vec<u8>> + 'a>
    where
        Self: 'a;
    type ValuesIterator<'a>
        = Box<dyn Iterator<Item = Vec<u8>> + 'a>
    where
        Self: 'a;
    type PairsIterator<'a>
        = Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + 'a>
    where
        Self: 'a;

    fn keys<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::KeysIterator<'a> {
        let (start, end) = bounds_to_option(start, end);

        self.0.range_keys(
            start.as_deref(),
            end.as_deref(),
            cosmwasm_std::Order::Ascending,
        )
    }

    fn values<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::ValuesIterator<'a> {
        let (start, end) = bounds_to_option(start, end);

        self.0.range_values(
            start.as_deref(),
            end.as_deref(),
            cosmwasm_std::Order::Ascending,
        )
    }

    fn pairs<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::PairsIterator<'a> {
        let (start, end) = bounds_to_option(start, end);

        self.0.range(
            start.as_deref(),
            end.as_deref(),
            cosmwasm_std::Order::Ascending,
        )
    }
}

impl<S> RevIterableStorage for CwStorage<S>
where
    S: cosmwasm_std::Storage + ?Sized,
{
    type RevKeysIterator<'a>
        = Box<dyn Iterator<Item = Vec<u8>> + 'a>
    where
        Self: 'a;
    type RevValuesIterator<'a>
        = Box<dyn Iterator<Item = Vec<u8>> + 'a>
    where
        Self: 'a;
    type RevPairsIterator<'a>
        = Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + 'a>
    where
        Self: 'a;

    fn rev_keys<'a>(&'a self, start: Bound<&[u8]>, end: Bound<&[u8]>) -> Self::RevKeysIterator<'a> {
        let (start, end) = bounds_to_option(start, end);

        self.0.range_keys(
            start.as_deref(),
            end.as_deref(),
            cosmwasm_std::Order::Descending,
        )
    }

    fn rev_values<'a>(
        &'a self,
        start: Bound<&[u8]>,
        end: Bound<&[u8]>,
    ) -> Self::RevValuesIterator<'a> {
        let (start, end) = bounds_to_option(start, end);

        self.0.range_values(
            start.as_deref(),
            end.as_deref(),
            cosmwasm_std::Order::Descending,
        )
    }

    fn rev_pairs<'a>(
        &'a self,
        start: Bound<&[u8]>,
        end: Bound<&[u8]>,
    ) -> Self::RevPairsIterator<'a> {
        let (start, end) = bounds_to_option(start, end);

        self.0.range(
            start.as_deref(),
            end.as_deref(),
            cosmwasm_std::Order::Descending,
        )
    }
}

fn bounds_to_option(start: Bound<&[u8]>, end: Bound<&[u8]>) -> (Option<Vec<u8>>, Option<Vec<u8>>) {
    let start = match start {
        Bound::Included(key) => Some(key.to_vec()),
        Bound::Excluded(key) => {
            let mut key = key.to_vec();
            key.push(0);
            Some(key)
        }
        Bound::Unbounded => None,
    };

    let end = match end {
        Bound::Included(key) => {
            let mut key = key.to_vec();
            key.push(0);
            Some(key)
        }
        Bound::Excluded(key) => Some(key.to_vec()),
        Bound::Unbounded => None,
    };

    (start, end)
}

impl<'a, S> IntoStorage<&'a CwStorage<S>> for (&'a S,)
where
    S: cosmwasm_std::Storage + ?Sized,
{
    fn into_storage(self) -> &'a CwStorage<S> {
        // Safety: `CwStorage` is a transparent wrapper around `S`, guaranteed to have the same
        // memory layout.
        //
        // The data in memory is the same and the lifetime is preserved.
        //
        // The same strategy is used in `std::Path`. Look it up!
        unsafe { &*(self.0 as *const _ as *const CwStorage<S>) }
    }
}

impl<'a, S> IntoStorage<&'a mut CwStorage<S>> for (&'a mut S,)
where
    S: cosmwasm_std::Storage + ?Sized,
{
    fn into_storage(self) -> &'a mut CwStorage<S> {
        // Safety: `CwStorage` is a transparent wrapper around `S`, guaranteed to have the same
        // memory layout.
        //
        // The data in memory is the same and the lifetime is preserved.
        //
        // The same strategy is used in `std::Path`. Look it up!
        unsafe { &mut *(self.0 as *mut _ as *mut CwStorage<S>) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_remove() {
        let cw_storage = cosmwasm_std::MemoryStorage::new();
        let mut storage = CwStorage(cw_storage);

        storage.set(b"key1", b"value1");
        storage.set(b"key2", b"value2");
        storage.set(b"key3", b"value3");

        storage.remove(b"key2");

        let keys: Vec<Vec<u8>> = storage.keys(Bound::Unbounded, Bound::Unbounded).collect();
        assert_eq!(keys, vec![b"key1".to_vec(), b"key3".to_vec()]);
    }

    #[test]
    fn keys_bounded() {
        let cw_storage = cosmwasm_std::MemoryStorage::new();
        let mut storage = CwStorage(cw_storage);

        storage.set(b"key1", b"value1");
        storage.set(b"key2", b"value2");
        storage.set(b"key3", b"value3");

        let keys: Vec<Vec<u8>> = storage
            .keys(Bound::Included(b"key1"), Bound::Excluded(b"key3"))
            .collect();
        assert_eq!(keys, vec![b"key1".to_vec(), b"key2".to_vec()]);

        let keys: Vec<Vec<u8>> = storage
            .keys(Bound::Excluded(b"key1"), Bound::Included(b"key3"))
            .collect();
        assert_eq!(keys, vec![b"key2".to_vec(), b"key3".to_vec()]);
    }

    #[test]
    fn keys_unbounded() {
        let cw_storage = cosmwasm_std::MemoryStorage::new();
        let mut storage = CwStorage(cw_storage);

        storage.set(b"key1", b"value1");
        storage.set(b"key2", b"value2");
        storage.set(b"key3", b"value3");

        let keys: Vec<Vec<u8>> = storage.keys(Bound::Unbounded, Bound::Unbounded).collect();
        assert_eq!(
            keys,
            vec![b"key1".to_vec(), b"key2".to_vec(), b"key3".to_vec()]
        );

        let keys: Vec<Vec<u8>> = storage
            .keys(Bound::Unbounded, Bound::Excluded(b"key3"))
            .collect();
        assert_eq!(keys, vec![b"key1".to_vec(), b"key2".to_vec()]);
    }

    #[test]
    fn values_bounded() {
        let cw_storage = cosmwasm_std::MemoryStorage::new();
        let mut storage = CwStorage(cw_storage);

        storage.set(b"key1", b"value1");
        storage.set(b"key2", b"value2");
        storage.set(b"key3", b"value3");

        let values: Vec<Vec<u8>> = storage
            .values(Bound::Included(b"key1"), Bound::Excluded(b"key3"))
            .collect();
        assert_eq!(values, vec![b"value1".to_vec(), b"value2".to_vec()]);

        let values: Vec<Vec<u8>> = storage
            .values(Bound::Excluded(b"key1"), Bound::Included(b"key3"))
            .collect();
        assert_eq!(values, vec![b"value2".to_vec(), b"value3".to_vec()]);
    }

    #[test]
    fn values_unbounded() {
        let cw_storage = cosmwasm_std::MemoryStorage::new();
        let mut storage = CwStorage(cw_storage);

        storage.set(b"key1", b"value1");
        storage.set(b"key2", b"value2");
        storage.set(b"key3", b"value3");

        let values: Vec<Vec<u8>> = storage.values(Bound::Unbounded, Bound::Unbounded).collect();
        assert_eq!(
            values,
            vec![b"value1".to_vec(), b"value2".to_vec(), b"value3".to_vec()]
        );

        let values: Vec<Vec<u8>> = storage
            .values(Bound::Unbounded, Bound::Excluded(b"key3"))
            .collect();
        assert_eq!(values, vec![b"value1".to_vec(), b"value2".to_vec()]);
    }

    #[test]
    fn pairs_bounded() {
        let cw_storage = cosmwasm_std::MemoryStorage::new();
        let mut storage = CwStorage(cw_storage);

        storage.set(b"key1", b"value1");
        storage.set(b"key2", b"value2");
        storage.set(b"key3", b"value3");

        let pairs: Vec<(Vec<u8>, Vec<u8>)> = storage
            .pairs(Bound::Included(b"key1"), Bound::Excluded(b"key3"))
            .collect();
        assert_eq!(
            pairs,
            vec![
                (b"key1".to_vec(), b"value1".to_vec()),
                (b"key2".to_vec(), b"value2".to_vec())
            ]
        );

        let pairs: Vec<(Vec<u8>, Vec<u8>)> = storage
            .pairs(Bound::Excluded(b"key1"), Bound::Included(b"key3"))
            .collect();
        assert_eq!(
            pairs,
            vec![
                (b"key2".to_vec(), b"value2".to_vec()),
                (b"key3".to_vec(), b"value3".to_vec())
            ]
        );
    }

    #[test]
    fn pairs_unbounded() {
        let cw_storage = cosmwasm_std::MemoryStorage::new();
        let mut storage = CwStorage(cw_storage);

        storage.set(b"key1", b"value1");
        storage.set(b"key2", b"value2");
        storage.set(b"key3", b"value3");

        let pairs: Vec<(Vec<u8>, Vec<u8>)> =
            storage.pairs(Bound::Unbounded, Bound::Unbounded).collect();
        assert_eq!(
            pairs,
            vec![
                (b"key1".to_vec(), b"value1".to_vec()),
                (b"key2".to_vec(), b"value2".to_vec()),
                (b"key3".to_vec(), b"value3".to_vec())
            ]
        );

        let pairs: Vec<(Vec<u8>, Vec<u8>)> = storage
            .pairs(Bound::Unbounded, Bound::Excluded(b"key3"))
            .collect();
        assert_eq!(
            pairs,
            vec![
                (b"key1".to_vec(), b"value1".to_vec()),
                (b"key2".to_vec(), b"value2".to_vec())
            ]
        );
    }

    #[test]
    fn rev_keys_bounded() {
        let cw_storage = cosmwasm_std::MemoryStorage::new();
        let mut storage = CwStorage(cw_storage);

        storage.set(b"key1", b"value1");
        storage.set(b"key2", b"value2");
        storage.set(b"key3", b"value3");

        let keys: Vec<Vec<u8>> = storage
            .rev_keys(Bound::Included(b"key1"), Bound::Excluded(b"key3"))
            .collect();
        assert_eq!(keys, vec![b"key2".to_vec(), b"key1".to_vec()]);

        let keys: Vec<Vec<u8>> = storage
            .rev_keys(Bound::Excluded(b"key1"), Bound::Included(b"key3"))
            .collect();
        assert_eq!(keys, vec![b"key3".to_vec(), b"key2".to_vec()]);
    }

    #[test]
    fn rev_keys_unbounded() {
        let cw_storage = cosmwasm_std::MemoryStorage::new();
        let mut storage = CwStorage(cw_storage);

        storage.set(b"key1", b"value1");
        storage.set(b"key2", b"value2");
        storage.set(b"key3", b"value3");

        let keys: Vec<Vec<u8>> = storage
            .rev_keys(Bound::Unbounded, Bound::Unbounded)
            .collect();
        assert_eq!(
            keys,
            vec![b"key3".to_vec(), b"key2".to_vec(), b"key1".to_vec()]
        );

        let keys: Vec<Vec<u8>> = storage
            .rev_keys(Bound::Unbounded, Bound::Excluded(b"key3"))
            .collect();
        assert_eq!(keys, vec![b"key2".to_vec(), b"key1".to_vec()]);
    }

    #[test]
    fn rev_values_bounded() {
        let cw_storage = cosmwasm_std::MemoryStorage::new();
        let mut storage = CwStorage(cw_storage);

        storage.set(b"key1", b"value1");
        storage.set(b"key2", b"value2");
        storage.set(b"key3", b"value3");

        let values: Vec<Vec<u8>> = storage
            .rev_values(Bound::Included(b"key1"), Bound::Excluded(b"key3"))
            .collect();
        assert_eq!(values, vec![b"value2".to_vec(), b"value1".to_vec()]);

        let values: Vec<Vec<u8>> = storage
            .rev_values(Bound::Excluded(b"key1"), Bound::Included(b"key3"))
            .collect();
        assert_eq!(values, vec![b"value3".to_vec(), b"value2".to_vec()]);
    }

    #[test]
    fn rev_values_unbounded() {
        let cw_storage = cosmwasm_std::MemoryStorage::new();
        let mut storage = CwStorage(cw_storage);

        storage.set(b"key1", b"value1");
        storage.set(b"key2", b"value2");
        storage.set(b"key3", b"value3");

        let values: Vec<Vec<u8>> = storage
            .rev_values(Bound::Unbounded, Bound::Unbounded)
            .collect();
        assert_eq!(
            values,
            vec![b"value3".to_vec(), b"value2".to_vec(), b"value1".to_vec()]
        );

        let values: Vec<Vec<u8>> = storage
            .rev_values(Bound::Unbounded, Bound::Excluded(b"key3"))
            .collect();
        assert_eq!(values, vec![b"value2".to_vec(), b"value1".to_vec()]);
    }

    #[test]
    fn rev_pairs_bounded() {
        let cw_storage = cosmwasm_std::MemoryStorage::new();
        let mut storage = CwStorage(cw_storage);

        storage.set(b"key1", b"value1");
        storage.set(b"key2", b"value2");
        storage.set(b"key3", b"value3");

        let pairs: Vec<(Vec<u8>, Vec<u8>)> = storage
            .rev_pairs(Bound::Included(b"key1"), Bound::Excluded(b"key3"))
            .collect();
        assert_eq!(
            pairs,
            vec![
                (b"key2".to_vec(), b"value2".to_vec()),
                (b"key1".to_vec(), b"value1".to_vec())
            ]
        );

        let pairs: Vec<(Vec<u8>, Vec<u8>)> = storage
            .rev_pairs(Bound::Excluded(b"key1"), Bound::Included(b"key3"))
            .collect();
        assert_eq!(
            pairs,
            vec![
                (b"key3".to_vec(), b"value3".to_vec()),
                (b"key2".to_vec(), b"value2".to_vec())
            ]
        );
    }

    #[test]
    fn rev_pairs_unbounded() {
        let cw_storage = cosmwasm_std::MemoryStorage::new();
        let mut storage = CwStorage(cw_storage);

        storage.set(b"key1", b"value1");
        storage.set(b"key2", b"value2");
        storage.set(b"key3", b"value3");

        let pairs: Vec<(Vec<u8>, Vec<u8>)> = storage
            .rev_pairs(Bound::Unbounded, Bound::Unbounded)
            .collect();
        assert_eq!(
            pairs,
            vec![
                (b"key3".to_vec(), b"value3".to_vec()),
                (b"key2".to_vec(), b"value2".to_vec()),
                (b"key1".to_vec(), b"value1".to_vec())
            ]
        );

        let pairs: Vec<(Vec<u8>, Vec<u8>)> = storage
            .rev_pairs(Bound::Unbounded, Bound::Excluded(b"key3"))
            .collect();
        assert_eq!(
            pairs,
            vec![
                (b"key2".to_vec(), b"value2".to_vec()),
                (b"key1".to_vec(), b"value1".to_vec())
            ]
        );
    }

    #[test]
    fn into_storage() {
        use cosmwasm_std::Storage as _;

        let mut storage = cosmwasm_std::MemoryStorage::new();
        storage.set(b"key", b"value");

        let storage_ref = (&storage,).into_storage();
        assert_eq!(storage_ref.get(b"key"), Some(b"value".to_vec()));
    }

    #[test]
    fn into_storage_mut() {
        use cosmwasm_std::Storage as _;

        let mut storage = cosmwasm_std::MemoryStorage::new();
        let storage_ref = (&mut storage,).into_storage();

        storage_ref.set(b"key", b"value");

        assert_eq!(storage.get(b"key"), Some(b"value".to_vec()));
    }
}

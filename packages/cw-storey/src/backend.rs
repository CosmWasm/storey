use std::ops::Bound;

use storey::storage::{IterableStorage, RevIterableStorage, StorageBackend, StorageBackendMut};

/// A wrapper around a type implementing [`cosmwasm_std::Storage`] that integrates it with [`storey`].
pub struct CwStorage<S>(pub S);

impl<S> StorageBackend for CwStorage<&S>
where
    S: cosmwasm_std::Storage + ?Sized,
{
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        cosmwasm_std::Storage::get(self.0, key)
    }
}

impl<S> StorageBackend for CwStorage<&mut S>
where
    S: cosmwasm_std::Storage + ?Sized,
{
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        cosmwasm_std::Storage::get(self.0, key)
    }
}

impl<S> StorageBackendMut for CwStorage<&mut S>
where
    S: cosmwasm_std::Storage + ?Sized,
{
    fn set(&mut self, key: &[u8], value: &[u8]) {
        cosmwasm_std::Storage::set(self.0, key, value)
    }

    fn remove(&mut self, key: &[u8]) {
        cosmwasm_std::Storage::remove(self.0, key)
    }
}

impl<S> IterableStorage for CwStorage<&S>
where
    S: cosmwasm_std::Storage + ?Sized,
{
    type KeysIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type ValuesIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type PairsIterator<'a> = Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + 'a> where Self: 'a;

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

impl<S> IterableStorage for CwStorage<&mut S>
where
    S: cosmwasm_std::Storage + ?Sized,
{
    type KeysIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type ValuesIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type PairsIterator<'a> = Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + 'a> where Self: 'a;

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

impl<S> RevIterableStorage for CwStorage<&S>
where
    S: cosmwasm_std::Storage + ?Sized,
{
    type RevKeysIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type RevValuesIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type RevPairsIterator<'a> = Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + 'a> where Self: 'a;

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

impl<S> RevIterableStorage for CwStorage<&mut S>
where
    S: cosmwasm_std::Storage + ?Sized,
{
    type RevKeysIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type RevValuesIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a> where Self: 'a;
    type RevPairsIterator<'a> = Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + 'a> where Self: 'a;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounds() {
        let mut cw_storage = cosmwasm_std::MemoryStorage::new();
        let mut storage = CwStorage(&mut cw_storage);

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
    fn test_unbounded() {
        let mut cw_storage = cosmwasm_std::MemoryStorage::new();
        let mut storage = CwStorage(&mut cw_storage);

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
}

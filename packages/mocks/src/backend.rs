use std::{cell::UnsafeCell, collections::BTreeMap};

use storey_storage::{IterableStorage, RevIterableStorage, StorageBackend, StorageBackendMut};

// `UnsafeCell` is needed here to implement interior mutability.
// https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
//
// We could play it safe and use `RefCell` instead, but we really don't need
// the extra bookkeeping. We're not trying to push borrow rules for the inner
// `BTreeMap` to runtime; we guarantee memory safety around aliasing at compile
// time.

pub struct TestStorage(UnsafeCell<BTreeMap<Vec<u8>, Vec<u8>>>);

impl TestStorage {
    pub fn new() -> Self {
        Self(UnsafeCell::new(BTreeMap::new()))
    }
}

impl Default for TestStorage {
    fn default() -> Self {
        Self::new()
    }
}

// Safety: in each of the unsafe blocks in this file, we drop the reference to
// the BTreeMap before the function returns, so we can guarantee that no two references
// to it exist at the same time.
//
// Moreover, we can further guarantee that the dereference is valid because the data
// is always initialized during construction.

impl StorageBackend for TestStorage {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        // Safety: see above
        unsafe { (*self.0.get()).get(key).cloned() }
    }
}

impl StorageBackendMut for TestStorage {
    fn set(&mut self, key: &[u8], value: &[u8]) {
        // Safety: see above
        unsafe {
            (*self.0.get()).insert(key.to_vec(), value.to_vec());
        }
    }

    fn remove(&mut self, key: &[u8]) {
        // Safety: see above
        unsafe {
            (*self.0.get()).remove(key);
        }
    }
}

impl IterableStorage for TestStorage {
    type KeysIterator<'a> = Box<dyn DoubleEndedIterator<Item = Vec<u8>> + 'a>;
    type ValuesIterator<'a> = Box<dyn DoubleEndedIterator<Item = Vec<u8>> + 'a>;
    type PairsIterator<'a> = Box<dyn DoubleEndedIterator<Item = (Vec<u8>, Vec<u8>)> + 'a>;

    fn keys<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::KeysIterator<'a> {
        let start = start.map(|x| x.to_vec());
        let end = end.map(|x| x.to_vec());

        Box::new(
            // Safety: see above
            unsafe { (*self.0.get()).clone() }
                .into_iter()
                .filter(move |(k, _)| check_bounds(k, start.as_ref(), end.as_ref()))
                .map(|(k, _)| k),
        )
    }

    fn values<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::ValuesIterator<'a> {
        let start = start.map(|x| x.to_vec());
        let end = end.map(|x| x.to_vec());

        Box::new(
            // Safety: see above
            unsafe { (*self.0.get()).clone() }
                .into_iter()
                .filter(move |(k, _)| check_bounds(k, start.as_ref(), end.as_ref()))
                .map(|(_, v)| v),
        )
    }

    fn pairs<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::PairsIterator<'a> {
        let start = start.map(|x| x.to_vec());
        let end = end.map(|x| x.to_vec());

        Box::new(
            // Safety: see above
            unsafe { (*self.0.get()).clone() }
                .into_iter()
                .filter(move |(k, _)| check_bounds(k, start.as_ref(), end.as_ref())),
        )
    }
}

impl RevIterableStorage for TestStorage {
    type RevKeysIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a>;
    type RevValuesIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a>;
    type RevPairsIterator<'a> = Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + 'a>;

    fn rev_keys<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevKeysIterator<'a> {
        Box::new(self.keys(start, end).rev())
    }

    fn rev_values<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevValuesIterator<'a> {
        Box::new(self.values(start, end).rev())
    }

    fn rev_pairs<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
    ) -> Self::RevPairsIterator<'a> {
        Box::new(self.pairs(start, end).rev())
    }
}

fn check_bounds(v: &[u8], start: Option<&Vec<u8>>, end: Option<&Vec<u8>>) -> bool {
    if let Some(start) = start {
        if v < start {
            return false;
        }
    }
    if let Some(end) = end {
        if v >= end {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_backend() {
        // TODO: split this into smaller tests?

        let mut storage = TestStorage::new();

        storage.set(&[0], b"bar");
        storage.set(&[1], b"baz");
        storage.set(&[1, 0], b"qux");
        storage.set(&[1, 1], b"quux");
        storage.set(&[2], b"qux");

        let keys: Vec<_> = storage.keys(None, None).collect();
        assert_eq!(
            keys,
            vec![vec![0], vec![1], vec![1, 0], vec![1, 1], vec![2]]
        );

        let some_keys: Vec<_> = storage.keys(Some(&[1]), Some(&[2])).collect();
        assert_eq!(some_keys, vec![vec![1], vec![1, 0], vec![1, 1]]);

        let values: Vec<_> = storage.values(None, None).collect();
        assert_eq!(
            values.iter().collect::<Vec<_>>(),
            vec![&b"bar"[..], b"baz", b"qux", b"quux", b"qux"]
        );

        let some_values: Vec<_> = storage.values(Some(&[1]), Some(&[2])).collect();
        assert_eq!(
            some_values.iter().collect::<Vec<_>>(),
            vec![&b"baz"[..], b"qux", b"quux"]
        );

        let pairs: Vec<_> = storage.pairs(None, None).collect();
        assert_eq!(
            pairs,
            vec![
                (vec![0], b"bar".to_vec()),
                (vec![1], b"baz".to_vec()),
                (vec![1, 0], b"qux".to_vec()),
                (vec![1, 1], b"quux".to_vec()),
                (vec![2], b"qux".to_vec()),
            ]
        );

        let some_pairs: Vec<_> = storage.pairs(Some(&[1]), Some(&[2])).collect();
        assert_eq!(
            some_pairs,
            vec![
                (vec![1], b"baz".to_vec()),
                (vec![1, 0], b"qux".to_vec()),
                (vec![1, 1], b"quux".to_vec()),
            ]
        );

        let rev_keys: Vec<_> = storage.rev_keys(None, None).collect();
        assert_eq!(
            rev_keys,
            vec![vec![2], vec![1, 1], vec![1, 0], vec![1], vec![0]]
        );

        let some_rev_keys: Vec<_> = storage.rev_keys(Some(&[1]), Some(&[2])).collect();
        assert_eq!(some_rev_keys, vec![vec![1, 1], vec![1, 0], vec![1]]);

        let rev_values: Vec<_> = storage.rev_values(None, None).collect();
        assert_eq!(
            rev_values.iter().collect::<Vec<_>>(),
            vec![&b"qux"[..], b"quux", b"qux", b"baz", b"bar"]
        );

        let some_rev_values: Vec<_> = storage.rev_values(Some(&[1]), Some(&[2])).collect();
        assert_eq!(
            some_rev_values.iter().collect::<Vec<_>>(),
            vec![&b"quux"[..], b"qux", b"baz"]
        );

        let rev_pairs: Vec<_> = storage.rev_pairs(None, None).collect();
        assert_eq!(
            rev_pairs,
            vec![
                (vec![2], b"qux".to_vec()),
                (vec![1, 1], b"quux".to_vec()),
                (vec![1, 0], b"qux".to_vec()),
                (vec![1], b"baz".to_vec()),
                (vec![0], b"bar".to_vec()),
            ]
        );

        let some_rev_pairs: Vec<_> = storage.rev_pairs(Some(&[1]), Some(&[2])).collect();
        assert_eq!(
            some_rev_pairs,
            vec![
                (vec![1, 1], b"quux".to_vec()),
                (vec![1, 0], b"qux".to_vec()),
                (vec![1], b"baz".to_vec()),
            ]
        );
    }

    #[test]
    fn metadata() {
        use storey_storage::StorageMut as _;

        let mut storage = TestStorage::new();
        storage.set_meta(&[0], b"meta");

        assert_eq!(StorageBackend::get(&storage, &[0]), None);
        assert_eq!(
            StorageBackend::get(&storage, &[255, 0]),
            Some(b"meta".to_vec())
        );
    }
}

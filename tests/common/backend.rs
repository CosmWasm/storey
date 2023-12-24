use std::{cell::UnsafeCell, collections::BTreeMap};

use stork::StorageIterableBackend as _;

pub struct TestStorage(UnsafeCell<BTreeMap<Vec<u8>, Vec<u8>>>);

impl TestStorage {
    pub fn new() -> Self {
        Self(UnsafeCell::new(BTreeMap::new()))
    }
}

// Safety: in each of the following methods, we drop the reference to the BTreeMap
// before the function returns, so we can guarantee that no two references exist
// at the same time.
//
// Moreover, we can further guarantee that the dereference is valid because the data
// is always initialized during construction.
impl stork::StorageBackend for TestStorage {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        unsafe { (&*self.0.get()).get(key).map(|v| v.clone()) }
    }

    fn set(&self, key: &[u8], value: &[u8]) {
        unsafe {
            (&mut *self.0.get()).insert(key.to_vec(), value.to_vec());
        }
    }

    fn remove(&self, key: &[u8]) {
        unsafe {
            (&mut *self.0.get()).remove(key);
        }
    }
}

impl stork::StorageIterableBackend for TestStorage {
    type KeysIterator<'a> = Box<dyn DoubleEndedIterator<Item = Vec<u8>> + 'a>;
    type ValuesIterator<'a> = Box<dyn DoubleEndedIterator<Item = Vec<u8>> + 'a>;
    type PairsIterator<'a> = Box<dyn DoubleEndedIterator<Item = (Vec<u8>, Vec<u8>)> + 'a>;

    fn keys<'a>(
        &'a self,
        start: Option<&'a [u8]>,
        end: Option<&'a [u8]>,
    ) -> Self::KeysIterator<'a> {
        Box::new(
            unsafe { (&*self.0.get()).clone() }
                .into_iter()
                .filter(move |(k, _)| check_bounds(k, start, end))
                .map(|(k, _)| k),
        )
    }

    fn values<'a>(
        &'a self,
        start: Option<&'a [u8]>,
        end: Option<&'a [u8]>,
    ) -> Self::ValuesIterator<'a> {
        Box::new(
            unsafe { (&*self.0.get()).clone() }
                .into_iter()
                .filter(move |(k, _)| check_bounds(k, start, end))
                .map(|(_, v)| v),
        )
    }

    fn pairs<'a>(
        &'a self,
        start: Option<&'a [u8]>,
        end: Option<&'a [u8]>,
    ) -> Self::PairsIterator<'a> {
        Box::new(
            unsafe { (&*self.0.get()).clone() }
                .into_iter()
                .filter(move |(k, _)| check_bounds(k, start, end)),
        )
    }
}

impl stork::StorageRevIterableBackend for TestStorage {
    type RevKeysIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a>;
    type RevValuesIterator<'a> = Box<dyn Iterator<Item = Vec<u8>> + 'a>;
    type RevPairsIterator<'a> = Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + 'a>;

    fn rev_keys<'a>(
        &'a self,
        start: Option<&'a [u8]>,
        end: Option<&'a [u8]>,
    ) -> Self::RevKeysIterator<'a> {
        Box::new(self.keys(start, end).rev())
    }

    fn rev_values<'a>(
        &'a self,
        start: Option<&'a [u8]>,
        end: Option<&'a [u8]>,
    ) -> Self::RevValuesIterator<'a> {
        Box::new(self.values(start, end).rev())
    }

    fn rev_pairs<'a>(
        &'a self,
        start: Option<&'a [u8]>,
        end: Option<&'a [u8]>,
    ) -> Self::RevPairsIterator<'a> {
        Box::new(self.pairs(start, end).rev())
    }
}

fn check_bounds(v: &[u8], start: Option<&[u8]>, end: Option<&[u8]>) -> bool {
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

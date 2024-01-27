use std::{cell::UnsafeCell, collections::BTreeMap};

use stork::IterableStorage as _;

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

// Safety: in each of the unsafe blocks in this file, we drop the reference to
// the BTreeMap before the function returns, so we can guarantee that no two references
// to it exist at the same time.
//
// Moreover, we can further guarantee that the dereference is valid because the data
// is always initialized during construction.

impl stork::Storage for TestStorage {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        // Safety: see above
        unsafe { (&*self.0.get()).get(key).map(|v| v.clone()) }
    }
}

impl stork::StorageMut for TestStorage {
    fn set(&self, key: &[u8], value: &[u8]) {
        // Safety: see above
        unsafe {
            (&mut *self.0.get()).insert(key.to_vec(), value.to_vec());
        }
    }

    fn remove(&self, key: &[u8]) {
        // Safety: see above
        unsafe {
            (&mut *self.0.get()).remove(key);
        }
    }
}

impl stork::IterableStorage for TestStorage {
    type KeysIterator<'a> = Box<dyn DoubleEndedIterator<Item = Vec<u8>> + 'a>;
    type ValuesIterator<'a> = Box<dyn DoubleEndedIterator<Item = Vec<u8>> + 'a>;
    type PairsIterator<'a> = Box<dyn DoubleEndedIterator<Item = (Vec<u8>, Vec<u8>)> + 'a>;

    fn keys<'a>(&'a self, start: Option<&[u8]>, end: Option<&[u8]>) -> Self::KeysIterator<'a> {
        let start = start.map(|x| x.to_vec());
        let end = end.map(|x| x.to_vec());

        Box::new(
            // Safety: see above
            unsafe { (&*self.0.get()).clone() }
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
            unsafe { (&*self.0.get()).clone() }
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
            unsafe { (&*self.0.get()).clone() }
                .into_iter()
                .filter(move |(k, _)| check_bounds(k, start.as_ref(), end.as_ref())),
        )
    }
}

impl stork::RevIterableStorage for TestStorage {
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
